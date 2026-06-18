<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import {
    forceCenter,
    forceCollide,
    forceLink,
    forceManyBody,
    forceSimulation
  } from 'd3-force';
  import { Application, Container, Graphics, Text, TextStyle } from 'pixi.js';

  import type { Simulation } from 'd3-force';
  import type { GraphData, TodoItem } from '../../types';
  import type { GraphCategoryCell, GraphSimLink, GraphSimNode } from '$lib/checklist/graphCanvasUtils';
  import {
    buildGraphSimulationData,
    hexToNum,
    readGraphThemeColors,
    truncateGraphLabel,
    GRAPH_TAP_THRESHOLD
  } from '$lib/checklist/graphCanvasUtils';

  type MaybePromise<T = void> = T | Promise<T>;

  interface Props {
    data: GraphData;
    initialSelectedItemId?: number | null;
    onItemEdit: (itemId: number) => MaybePromise;
    onItemToggle: (itemId: number) => MaybePromise<TodoItem>;
  }

  let {
    data,
    initialSelectedItemId = null,
    onItemEdit,
    onItemToggle
  }: Props = $props();

  let canvasContainer: HTMLDivElement;
  let app: Application | null = null;
  let simulation: Simulation<GraphSimNode, GraphSimLink> | null = null;
  let animFrameId: number | null = null;
  let fitTimeoutIds: ReturnType<typeof setTimeout>[] = [];
  let destroyed = false;

  function categoryForce(categoryCells: GraphCategoryCell[]) {
    const cells = new Map(categoryCells.map((cell) => [cell.category.id, cell]));

    return (alpha: number): void => {
      if (!simulation) return;

      for (const node of simulation.nodes()) {
        if (node.nodeType !== 'item' || node.categoryId === null) continue;
        const cell = cells.get(node.categoryId);
        if (!cell) continue;

        const x = node.x ?? cell.centerX;
        const y = node.y ?? cell.centerY;

        node.vx = (node.vx ?? 0) + (cell.centerX - x) * 0.045 * alpha;
        node.vy = (node.vy ?? 0) + (cell.centerY - y) * 0.045 * alpha;
      }
    };
  }

  function tagForce(simLinks: GraphSimLink[]) {
    return (alpha: number): void => {
      const targets = new Map<
        string,
        { node: GraphSimNode; x: number; y: number; count: number }
      >();

      for (const link of simLinks) {
        const tagNode = link.source.nodeType === 'tag' ? link.source : link.target;
        const itemNode = link.source.nodeType === 'item' ? link.source : link.target;
        if (tagNode.nodeType !== 'tag' || itemNode.nodeType !== 'item') continue;

        const current = targets.get(tagNode.id) ?? { node: tagNode, x: 0, y: 0, count: 0 };
        current.x += itemNode.x ?? 0;
        current.y += itemNode.y ?? 0;
        current.count += 1;
        targets.set(tagNode.id, current);
      }

      for (const target of targets.values()) {
        if (target.count === 0) continue;

        const targetX = target.x / target.count;
        const targetY = target.y / target.count;
        const x = target.node.x ?? targetX;
        const y = target.node.y ?? targetY;
        target.node.vx = (target.node.vx ?? 0) + (targetX - x) * 0.12 * alpha;
        target.node.vy = (target.node.vy ?? 0) + (targetY - y) * 0.12 * alpha;
      }
    };
  }

  function connectedToTag(
    node: GraphSimNode,
    highlightedTagId: string | null,
    simLinks: GraphSimLink[]
  ): boolean {
    if (!highlightedTagId) return true;
    if (node.id === highlightedTagId) return true;

    return simLinks.some(
      (link) =>
        (link.source.id === highlightedTagId && link.target.id === node.id) ||
        (link.target.id === highlightedTagId && link.source.id === node.id)
    );
  }

  onMount(async () => {
    if (destroyed) return;

    const width = Math.max(320, canvasContainer.clientWidth);
    const height = Math.max(360, canvasContainer.clientHeight);
    const theme = readGraphThemeColors();

    app = new Application();
    await app.init({
      preference: 'webgl',
      width,
      height,
      backgroundAlpha: 0,
      antialias: true,
      resolution: window.devicePixelRatio || 1,
      autoDensity: true
    });

    if (destroyed) {
      app.destroy();
      app = null;
      return;
    }

    canvasContainer.appendChild(app.canvas as HTMLCanvasElement);

    const world = new Container();
    app.stage.addChild(world);
    const haloLayer = new Container();
    app.stage.addChild(haloLayer);

    const membraneGraphics = new Graphics();
    const edgeGraphics = new Graphics();
    const highlightEdgeGraphics = new Graphics();
    const nodeLayer = new Container();
    const textLayer = new Container();
    world.addChild(membraneGraphics);
    world.addChild(edgeGraphics);
    world.addChild(highlightEdgeGraphics);
    world.addChild(nodeLayer);
    world.addChild(textLayer);

    let highlightedTagId: string | null = null;
    let isNodeDragging = false;
    let hasUserTransformed = false;
    let selectedItemNode: GraphSimNode | null = null;
    let lastNodePointerUpAt = 0;
    let lastHaloPointerAt = 0;
    let haloActionInFlight = false;
    let haloAnimationStartedAt = 0;
    let haloDismissStartedAt = 0;
    let dismissingItemNode: GraphSimNode | null = null;
    const { categoryCells, simNodes, simLinks } = buildGraphSimulationData(data, width, height);
    selectedItemNode =
      initialSelectedItemId === null
        ? null
        : simNodes.find(
            (node) => node.nodeType === 'item' && node.rawId === initialSelectedItemId
          ) ?? null;

    const categoryTextStyle = new TextStyle({
      fontSize: 13,
      fontWeight: '700',
      fill: hexToNum(theme.ink)
    });
    const tagTextStyle = new TextStyle({
      fontSize: 11,
      fontWeight: '700',
      fill: hexToNum(theme.ink)
    });
    const itemTextStyle = new TextStyle({
      fontSize: 10,
      fontWeight: '700',
      fill: hexToNum(theme.inkMuted)
    });

    for (const cell of categoryCells) {
      const label = new Text({
        text: truncateGraphLabel(cell.category.name, 18),
        style: categoryTextStyle
      });
      label.anchor.set(0, 0);
      textLayer.addChild(label);
      cell.textObj = label;
    }

    function redrawNode(node: GraphSimNode): void {
      if (!node.graphics) return;
      const graphics = node.graphics;
      graphics.clear();

      if (node.nodeType === 'tag') {
        const width = Math.max(42, node.radius * 2);
        graphics.roundRect(-width / 2, -15, width, 30, 15);
        graphics.fill({ color: hexToNum(theme.sky), alpha: 0.9 });
        graphics.stroke({ color: hexToNum(theme.ink), width: 1.25, alpha: 0.45 });
        return;
      }

      const fillColor = node.done ? theme.peach : theme.white;
      const auraColor = node.done ? theme.peach : theme.sky;

      graphics.circle(0, 2, node.radius + 2.5);
      graphics.fill({ color: hexToNum(theme.ink), alpha: 0.07 });

      graphics.circle(0, 0, node.radius + 4);
      graphics.fill({ color: hexToNum(auraColor), alpha: node.done ? 0.18 : 0.2 });

      graphics.circle(0, 0, node.radius);
      graphics.fill({ color: hexToNum(fillColor), alpha: node.done ? 0.82 : 0.98 });
      graphics.stroke({ color: hexToNum(theme.ink), width: 1.45, alpha: node.done ? 0.42 : 0.5 });

      graphics.circle(-node.radius * 0.28, -node.radius * 0.32, Math.max(2.4, node.radius * 0.22));
      graphics.fill({ color: hexToNum(theme.white), alpha: node.done ? 0.28 : 0.58 });
    }

    const toggleHaloButton = new Graphics();
    const editHaloButton = new Graphics();
    haloLayer.addChild(toggleHaloButton);
    haloLayer.addChild(editHaloButton);

    function clamp(value: number, min: number, max: number): number {
      return Math.min(Math.max(value, min), max);
    }

    function prepareHaloButton(graphics: Graphics): void {
      graphics.eventMode = 'static';
      graphics.cursor = 'pointer';
      graphics.on('pointerdown', (event) => {
        event.stopPropagation();
        lastHaloPointerAt = Date.now();
      });
    }

    function drawHaloSurface(graphics: Graphics, x: number, y: number): void {
      graphics.clear();
      graphics.x = x;
      graphics.y = y;
      graphics.circle(0, 0, 17);
      graphics.fill({ color: hexToNum(theme.white), alpha: haloActionInFlight ? 0.64 : 0.96 });
      graphics.stroke({ color: hexToNum(theme.ink), width: 1.35, alpha: 0.62 });
    }

    function drawCheckIcon(graphics: Graphics): void {
      const scale = 0.78;
      graphics.moveTo((20 - 12) * scale, (6 - 12) * scale);
      graphics.lineTo((9 - 12) * scale, (17 - 12) * scale);
      graphics.lineTo((4 - 12) * scale, (12 - 12) * scale);
      graphics.stroke({
        color: hexToNum(theme.ink),
        width: 2.15,
        alpha: haloActionInFlight ? 0.48 : 0.88,
        cap: 'round',
        join: 'round'
      });
    }

    function drawUndoIcon(graphics: Graphics): void {
      const scale = 0.7;
      graphics.moveTo((9 - 12) * scale, (14 - 12) * scale);
      graphics.lineTo((4 - 12) * scale, (9 - 12) * scale);
      graphics.lineTo((9 - 12) * scale, (4 - 12) * scale);
      graphics.moveTo((4 - 12) * scale, (9 - 12) * scale);
      graphics.lineTo((14.5 - 12) * scale, (9 - 12) * scale);
      graphics.quadraticCurveTo(
        (20 - 12) * scale,
        (9 - 12) * scale,
        (20 - 12) * scale,
        (14.5 - 12) * scale
      );
      graphics.quadraticCurveTo(
        (20 - 12) * scale,
        (20 - 12) * scale,
        (14.5 - 12) * scale,
        (20 - 12) * scale
      );
      graphics.lineTo((11 - 12) * scale, (20 - 12) * scale);
      graphics.stroke({
        color: hexToNum(theme.ink),
        width: 2.1,
        alpha: haloActionInFlight ? 0.48 : 0.88,
        cap: 'round',
        join: 'round'
      });
    }

    function drawEditIcon(graphics: Graphics): void {
      graphics.moveTo(-7.4, 7.4);
      graphics.lineTo(-5.1, 2.3);
      graphics.lineTo(5.6, -8.4);
      graphics.quadraticCurveTo(6.9, -9.7, 8.2, -8.4);
      graphics.quadraticCurveTo(9.5, -7.1, 8.2, -5.8);
      graphics.lineTo(-2.5, 4.9);
      graphics.lineTo(-7.4, 7.4);
      graphics.moveTo(-5.1, 2.3);
      graphics.lineTo(-2.5, 4.9);
      graphics.stroke({
        color: hexToNum(theme.ink),
        width: 1.9,
        alpha: haloActionInFlight ? 0.48 : 0.88,
        cap: 'round',
        join: 'round'
      });
    }

    function startHaloDismiss(): void {
      if (!selectedItemNode) return;
      dismissingItemNode = selectedItemNode;
      selectedItemNode = null;
      haloDismissStartedAt = performance.now();
    }

    function clearHaloImmediately(): void {
      selectedItemNode = null;
      dismissingItemNode = null;
      haloDismissStartedAt = 0;
      haloLayer.visible = false;
      toggleHaloButton.clear();
      editHaloButton.clear();
    }

    function drawHalo(): void {
      const activeNode = selectedItemNode ?? dismissingItemNode;
      if (!activeNode) {
        haloLayer.visible = false;
        toggleHaloButton.clear();
        editHaloButton.clear();
        return;
      }

      haloLayer.visible = true;
      const scale = world.scale.x;
      const nodeX = activeNode.x ?? width / 2;
      const nodeY = activeNode.y ?? height / 2;
      const screenX = world.x + nodeX * scale;
      const screenY = world.y + nodeY * scale;
      const actionGap = 42;
      const buttonRadius = 17;
      const pairHalfWidth = actionGap / 2 + buttonRadius;
      const now = performance.now();
      const isDismissing = selectedItemNode === null && dismissingItemNode !== null;
      const elapsed = Math.max(
        0,
        now - (isDismissing ? haloDismissStartedAt : haloAnimationStartedAt)
      );
      const duration = isDismissing ? 140 : 180;
      const progress = Math.min(1, elapsed / duration);
      const easedProgress = 1 - Math.pow(1 - progress, 3);
      const haloAlpha = isDismissing ? 1 - easedProgress : 0.2 + easedProgress * 0.8;
      const haloScale = isDismissing
        ? 1 - easedProgress * 0.16
        : 0.78 + easedProgress * 0.22;

      if (isDismissing && progress >= 1) {
        clearHaloImmediately();
        return;
      }

      const centerX = clamp(screenX, pairHalfWidth + 10, width - pairHalfWidth - 10);
      const y = clamp(
        screenY - activeNode.radius * scale - 30,
        buttonRadius + 10,
        height - buttonRadius - 10
      );

      drawHaloSurface(toggleHaloButton, centerX - actionGap / 2, y);
      toggleHaloButton.alpha = haloAlpha;
      toggleHaloButton.scale.set(haloScale);
      if (activeNode.done) {
        drawUndoIcon(toggleHaloButton);
      } else {
        drawCheckIcon(toggleHaloButton);
      }

      drawHaloSurface(editHaloButton, centerX + actionGap / 2, y);
      editHaloButton.alpha = haloAlpha;
      editHaloButton.scale.set(haloScale);
      drawEditIcon(editHaloButton);
    }

    prepareHaloButton(toggleHaloButton);
    prepareHaloButton(editHaloButton);

    toggleHaloButton.on('pointerup', async (event) => {
      event.stopPropagation();
      lastHaloPointerAt = Date.now();
      if (!selectedItemNode || haloActionInFlight) return;

      const node = selectedItemNode;
      const previousDone = node.done;
      haloActionInFlight = true;
      node.done = !previousDone;
      redrawNode(node);
      drawHalo();
      try {
        const updatedItem = await onItemToggle(node.rawId);
        if (!destroyed) {
          node.done = updatedItem.done;
          redrawNode(node);
        }
      } catch (error) {
        node.done = previousDone;
        redrawNode(node);
        console.error('Failed to toggle graph item', error);
      } finally {
        haloActionInFlight = false;
        if (!destroyed) {
          drawHalo();
        }
      }
    });

    editHaloButton.on('pointerup', (event) => {
      event.stopPropagation();
      lastHaloPointerAt = Date.now();
      if (!selectedItemNode || haloActionInFlight) return;

      const node = selectedItemNode;
      selectedItemNode = null;
      drawHalo();
      void onItemEdit(node.rawId);
    });

    for (const node of simNodes) {
      const graphics = new Graphics();
      node.graphics = graphics;
      redrawNode(node);
      graphics.eventMode = 'static';
      graphics.cursor = 'pointer';
      nodeLayer.addChild(graphics);

      const text = new Text({
        text:
          node.nodeType === 'tag'
            ? `#${truncateGraphLabel(node.label, 10)}`
            : truncateGraphLabel(node.label, 12),
        style: node.nodeType === 'tag' ? tagTextStyle : itemTextStyle
      });
      text.anchor.set(0.5, node.nodeType === 'tag' ? 0.5 : 0);
      textLayer.addChild(text);
      node.textObj = text;

      let dragging = false;
      let didMove = false;
      let dragOffset = { x: 0, y: 0 };
      let downPos = { x: 0, y: 0 };

      graphics.on('pointerdown', (event) => {
        event.stopPropagation();
        dragging = true;
        didMove = false;
        isNodeDragging = true;
        const worldPos = world.toLocal(event.global);
        dragOffset = {
          x: worldPos.x - (node.x ?? 0),
          y: worldPos.y - (node.y ?? 0)
        };
        downPos = { x: event.global.x, y: event.global.y };
        node.fx = node.x;
        node.fy = node.y;
        if (node.nodeType === 'tag') {
          highlightedTagId = node.id;
        }
        if (node.nodeType === 'item' && selectedItemNode?.id !== node.id) {
          startHaloDismiss();
        }
        simulation?.alphaTarget(0.08).restart();
      });

      graphics.on('pointerover', () => {
        if (node.nodeType === 'tag') {
          highlightedTagId = node.id;
        }
      });

      graphics.on('pointerout', () => {
        if (!dragging && node.nodeType === 'tag') {
          highlightedTagId = null;
        }
      });

      const onMove = (event: any): void => {
        if (!dragging) return;
        const dx = event.global.x - downPos.x;
        const dy = event.global.y - downPos.y;
        if (Math.abs(dx) > GRAPH_TAP_THRESHOLD || Math.abs(dy) > GRAPH_TAP_THRESHOLD) {
          didMove = true;
          hasUserTransformed = true;
          startHaloDismiss();
        }
        const worldPos = world.toLocal(event.global);
        node.fx = worldPos.x - dragOffset.x;
        node.fy = worldPos.y - dragOffset.y;
      };

      const onUp = (): void => {
        if (!dragging) return;
        dragging = false;
        isNodeDragging = false;
        node.fx = null;
        node.fy = null;
        simulation?.alphaTarget(0);

        if (node.nodeType === 'tag') {
          highlightedTagId = null;
        }

        if (!didMove && node.nodeType === 'item') {
          if (selectedItemNode?.id === node.id) {
            startHaloDismiss();
          } else {
            selectedItemNode = node;
            dismissingItemNode = null;
            haloAnimationStartedAt = performance.now();
          }
          lastNodePointerUpAt = Date.now();
        }
      };

      graphics.on('globalpointermove', onMove);
      graphics.on('pointerup', onUp);
      graphics.on('pointerupoutside', onUp);
    }

    simulation = forceSimulation(simNodes)
      .force(
        'link',
        forceLink<GraphSimNode, GraphSimLink>(simLinks)
          .id((node) => node.id)
          .distance((link) => (link.target.nodeType === 'item' ? 74 : 62))
          .strength(0.38)
      )
      .force(
        'charge',
        forceManyBody<GraphSimNode>().strength((node) =>
          node.nodeType === 'tag' ? -120 : -70
        )
      )
      .force('center', forceCenter(width / 2, height / 2).strength(0.04))
      .force('category', categoryForce(categoryCells))
      .force('tag', tagForce(simLinks))
      .force(
        'collide',
        forceCollide<GraphSimNode>().radius((node) =>
          node.nodeType === 'tag' ? node.radius + 24 : node.radius + 15
        )
      )
      .alphaDecay(0.045);

    interface ContentBounds {
      left: number;
      right: number;
      top: number;
      bottom: number;
    }

    interface CategoryMembraneRect extends ContentBounds {
      width: number;
      height: number;
    }

    function includeBounds(bounds: ContentBounds | null, next: ContentBounds): ContentBounds {
      if (!bounds) return next;

      return {
        left: Math.min(bounds.left, next.left),
        right: Math.max(bounds.right, next.right),
        top: Math.min(bounds.top, next.top),
        bottom: Math.max(bounds.bottom, next.bottom)
      };
    }

    function getCategoryMembraneRect(cell: GraphCategoryCell): CategoryMembraneRect | null {
      const itemNodes = simNodes.filter(
        (node) => node.nodeType === 'item' && node.categoryId === cell.category.id
      );
      if (itemNodes.length === 0) return null;

      const xs = itemNodes.map((node) => node.x ?? cell.centerX);
      const ys = itemNodes.map((node) => node.y ?? cell.centerY);
      const minX = Math.min(...xs);
      const maxX = Math.max(...xs);
      const minY = Math.min(...ys);
      const maxY = Math.max(...ys);
      const paddingX = 68;
      const paddingY = 58;
      const membraneWidth = Math.max(156, maxX - minX + paddingX * 2);
      const membraneHeight = Math.max(112, maxY - minY + paddingY * 2);
      const left = (minX + maxX) / 2 - membraneWidth / 2;
      const top = (minY + maxY) / 2 - membraneHeight / 2;

      return {
        left,
        right: left + membraneWidth,
        top,
        bottom: top + membraneHeight,
        width: membraneWidth,
        height: membraneHeight
      }
    }

    function getGraphContentBounds(): ContentBounds | null {
      let bounds: ContentBounds | null = null;

      for (const cell of categoryCells) {
        const rect = getCategoryMembraneRect(cell);
        if (rect) {
          bounds = includeBounds(bounds, rect);
        }
      }

      for (const node of simNodes) {
        const x = node.x ?? width / 2;
        const y = node.y ?? height / 2;
        const labelWidth = node.nodeType === 'tag' ? Math.max(56, node.radius * 2 + 16) : 86;
        const labelHeight = node.nodeType === 'tag' ? 34 : node.radius + 26;

        bounds = includeBounds(bounds, {
          left: x - Math.max(node.radius + 8, labelWidth / 2),
          right: x + Math.max(node.radius + 8, labelWidth / 2),
          top: y - Math.max(node.radius + 8, 22),
          bottom: y + labelHeight
        });
      }

      return bounds;
    }

    function fitWorldToContent(): void {
      if (destroyed || !app || hasUserTransformed) return;

      const bounds = getGraphContentBounds();
      if (!bounds) return;

      const contentWidth = Math.max(1, bounds.right - bounds.left);
      const contentHeight = Math.max(1, bounds.bottom - bounds.top);
      const paddingX = Math.min(44, width * 0.1);
      const paddingY = Math.min(56, height * 0.1);
      const nextScale = Math.min(
        1.18,
        Math.max(
          0.32,
          Math.min((width - paddingX * 2) / contentWidth, (height - paddingY * 2) / contentHeight)
        )
      );

      world.scale.set(nextScale);
      world.x = width / 2 - ((bounds.left + bounds.right) / 2) * nextScale;
      world.y = height * 0.48 - ((bounds.top + bounds.bottom) / 2) * nextScale;
    }

    simulation.tick(180);
    fitWorldToContent();

    function drawMembranes(): void {
      membraneGraphics.clear();

      for (const cell of categoryCells) {
        const rect = getCategoryMembraneRect(cell);
        if (!rect) continue;

        membraneGraphics.roundRect(rect.left, rect.top, rect.width, rect.height, 34);
        membraneGraphics.fill({ color: hexToNum(theme.white), alpha: 0.26 });
        membraneGraphics.stroke({ color: hexToNum(theme.ink), width: 1.4, alpha: 0.22 });

        if (cell.textObj) {
          cell.textObj.x = rect.left + 16;
          cell.textObj.y = rect.top + 12;
        }
      }
    }

    function getNodeEdgeInset(node: GraphSimNode, unitX: number, unitY: number): number {
      if (node.nodeType === 'tag') {
        const halfWidth = Math.max(42, node.radius * 2) / 2;
        const halfHeight = 15;
        const xDistance = halfWidth / Math.max(Math.abs(unitX), 0.001);
        const yDistance = halfHeight / Math.max(Math.abs(unitY), 0.001);
        return Math.min(xDistance, yDistance) + 2;
      }

      return node.radius + 2;
    }

    function drawTrimmedEdge(graphics: Graphics, link: GraphSimLink): void {
      const sourceX = link.source.x ?? 0;
      const sourceY = link.source.y ?? 0;
      const targetX = link.target.x ?? 0;
      const targetY = link.target.y ?? 0;
      const deltaX = targetX - sourceX;
      const deltaY = targetY - sourceY;
      const distance = Math.hypot(deltaX, deltaY);
      if (distance < 2) return;

      const unitX = deltaX / distance;
      const unitY = deltaY / distance;
      const maxInset = distance * 0.42;
      const sourceInset = Math.min(maxInset, getNodeEdgeInset(link.source, unitX, unitY));
      const targetInset = Math.min(maxInset, getNodeEdgeInset(link.target, -unitX, -unitY));
      const startX = sourceX + unitX * sourceInset;
      const startY = sourceY + unitY * sourceInset;
      const endX = targetX - unitX * targetInset;
      const endY = targetY - unitY * targetInset;
      const curveDirection = (link.tagId + link.target.rawId) % 2 === 0 ? 1 : -1;
      const curveOffset = Math.min(18, Math.max(7, distance * 0.07)) * curveDirection;
      const controlX = (startX + endX) / 2 - unitY * curveOffset;
      const controlY = (startY + endY) / 2 + unitX * curveOffset;

      graphics.moveTo(startX, startY);
      graphics.quadraticCurveTo(controlX, controlY, endX, endY);
    }

    function drawEdges(): void {
      edgeGraphics.clear();
      highlightEdgeGraphics.clear();

      for (const link of simLinks) {
        const isHighlighted =
          highlightedTagId &&
          (link.source.id === highlightedTagId || link.target.id === highlightedTagId);
        const graphics = isHighlighted ? highlightEdgeGraphics : edgeGraphics;
        drawTrimmedEdge(graphics, link);
      }

      edgeGraphics.stroke({
        color: hexToNum(theme.inkMuted),
        width: 1.05,
        alpha: highlightedTagId ? 0.08 : 0.22
      });
      highlightEdgeGraphics.stroke({
        color: hexToNum(theme.skyStrong),
        width: 1.8,
        alpha: 0.62
      });
    }

    function render(): void {
      if (destroyed || !app) return;

      drawMembranes();
      drawEdges();

      for (const node of simNodes) {
        const connected = connectedToTag(node, highlightedTagId, simLinks);
        if (node.graphics) {
          node.graphics.x = node.x ?? 0;
          node.graphics.y = node.y ?? 0;
          node.graphics.alpha = connected ? 1 : 0.28;
        }
        if (node.textObj) {
          node.textObj.x = node.x ?? 0;
          node.textObj.y =
            node.nodeType === 'tag' ? node.y ?? 0 : (node.y ?? 0) + node.radius + 7;
          node.textObj.alpha = connected ? 1 : 0.28;
        }
      }

      drawHalo();
      animFrameId = requestAnimationFrame(render);
    }

    render();

    let isPanning = false;
    let panPointerId: number | null = null;
    let panStart = { x: 0, y: 0 };
    let worldStart = { x: 0, y: 0 };
    let panDidMove = false;
    const activePointers = new Set<number>();
    const canvas = app.canvas as HTMLCanvasElement;

    canvas.addEventListener('pointerdown', (event) => {
      if (Date.now() - lastHaloPointerAt < 120) return;
      activePointers.add(event.pointerId);
      if (isNodeDragging) return;
      if (activePointers.size === 1 && panPointerId === null) {
        isPanning = true;
        panPointerId = event.pointerId;
        panDidMove = false;
        startHaloDismiss();
        panStart = { x: event.clientX, y: event.clientY };
        worldStart = { x: world.x, y: world.y };
      } else {
        isPanning = false;
        panPointerId = null;
      }
    });

    canvas.addEventListener('pointermove', (event) => {
      if (!isPanning || event.pointerId !== panPointerId || isNodeDragging) return;
      if (
        Math.abs(event.clientX - panStart.x) > GRAPH_TAP_THRESHOLD ||
        Math.abs(event.clientY - panStart.y) > GRAPH_TAP_THRESHOLD
      ) {
        panDidMove = true;
        startHaloDismiss();
      }
      hasUserTransformed = true;
      world.x = worldStart.x + event.clientX - panStart.x;
      world.y = worldStart.y + event.clientY - panStart.y;
    });

    const stopPointer = (event: PointerEvent): void => {
      activePointers.delete(event.pointerId);
      if (event.pointerId === panPointerId) {
        if (!panDidMove && Date.now() - lastNodePointerUpAt > 80 && Date.now() - lastHaloPointerAt > 120) {
          startHaloDismiss();
        }
        isPanning = false;
        panPointerId = null;
        panDidMove = false;
      }
    };
    canvas.addEventListener('pointerup', stopPointer);
    canvas.addEventListener('pointerleave', stopPointer);
    canvas.addEventListener('pointercancel', stopPointer);

    canvas.addEventListener(
      'wheel',
      (event) => {
        event.preventDefault();
        hasUserTransformed = true;
        startHaloDismiss();
        const scaleFactor = event.deltaY > 0 ? 0.94 : 1.06;
        const nextScale = Math.min(Math.max(world.scale.x * scaleFactor, 0.35), 2.4);
        const rect = canvas.getBoundingClientRect();
        const px = event.clientX - rect.left;
        const py = event.clientY - rect.top;
        const before = {
          x: (px - world.x) / world.scale.x,
          y: (py - world.y) / world.scale.y
        };
        world.scale.set(nextScale);
        world.x = px - before.x * nextScale;
        world.y = py - before.y * nextScale;
      },
      { passive: false }
    );

    let lastTouchDistance = 0;
    let lastTouchCenter = { x: 0, y: 0 };
    canvas.addEventListener(
      'touchstart',
      (event) => {
        if (event.touches.length !== 2) return;
        const dx = event.touches[0].clientX - event.touches[1].clientX;
        const dy = event.touches[0].clientY - event.touches[1].clientY;
        lastTouchDistance = Math.sqrt(dx * dx + dy * dy);
        lastTouchCenter = {
          x: (event.touches[0].clientX + event.touches[1].clientX) / 2,
          y: (event.touches[0].clientY + event.touches[1].clientY) / 2
        };
      },
      { passive: true }
    );
    canvas.addEventListener(
      'touchmove',
      (event) => {
        if (event.touches.length !== 2) return;
        event.preventDefault();
        const dx = event.touches[0].clientX - event.touches[1].clientX;
        const dy = event.touches[0].clientY - event.touches[1].clientY;
        const distance = Math.sqrt(dx * dx + dy * dy);
        const center = {
          x: (event.touches[0].clientX + event.touches[1].clientX) / 2,
          y: (event.touches[0].clientY + event.touches[1].clientY) / 2
        };

        if (lastTouchDistance > 0) {
          hasUserTransformed = true;
          startHaloDismiss();
          const nextScale = Math.min(Math.max(world.scale.x * (distance / lastTouchDistance), 0.35), 2.4);
          const rect = canvas.getBoundingClientRect();
          const px = center.x - rect.left;
          const py = center.y - rect.top;
          const before = {
            x: (px - world.x) / world.scale.x,
            y: (py - world.y) / world.scale.y
          };
          world.scale.set(nextScale);
          world.x = px - before.x * nextScale + center.x - lastTouchCenter.x;
          world.y = py - before.y * nextScale + center.y - lastTouchCenter.y;
        }

        lastTouchDistance = distance;
        lastTouchCenter = center;
      },
      { passive: false }
    );
    canvas.addEventListener(
      'touchend',
      () => {
        lastTouchDistance = 0;
      },
      { passive: true }
    );
  });

  onDestroy(() => {
    destroyed = true;
    if (animFrameId !== null) {
      cancelAnimationFrame(animFrameId);
    }
    for (const timeoutId of fitTimeoutIds) {
      clearTimeout(timeoutId);
    }
    fitTimeoutIds = [];
    if (simulation) {
      simulation.stop();
      simulation = null;
    }
    if (app) {
      app.destroy(true);
      app = null;
    }
  });
</script>

<div bind:this={canvasContainer} class="graph-canvas" aria-hidden="true"></div>

<style>
  .graph-canvas {
    position: absolute;
    inset: 0;
    overflow: hidden;
    touch-action: none;
  }

  .graph-canvas :global(canvas) {
    display: block;
  }
</style>
