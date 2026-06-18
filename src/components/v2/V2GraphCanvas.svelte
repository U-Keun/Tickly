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
  import type { V2GraphData } from '../../types';
  import type { V2GraphCategoryCell, V2GraphSimLink, V2GraphSimNode } from '$lib/v2/v2GraphCanvasUtils';
  import {
    buildV2GraphSimulationData,
    hexToNum,
    readV2GraphThemeColors,
    truncateGraphLabel,
    V2_GRAPH_TAP_THRESHOLD
  } from '$lib/v2/v2GraphCanvasUtils';

  type MaybePromise = void | Promise<void>;

  interface Props {
    data: V2GraphData;
    onItemSelect: (itemId: number) => MaybePromise;
  }

  let { data, onItemSelect }: Props = $props();

  let canvasContainer: HTMLDivElement;
  let app: Application | null = null;
  let simulation: Simulation<V2GraphSimNode, V2GraphSimLink> | null = null;
  let animFrameId: number | null = null;
  let fitTimeoutIds: ReturnType<typeof setTimeout>[] = [];
  let destroyed = false;

  function categoryForce(categoryCells: V2GraphCategoryCell[]) {
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

  function tagForce(simLinks: V2GraphSimLink[]) {
    return (alpha: number): void => {
      const targets = new Map<
        string,
        { node: V2GraphSimNode; x: number; y: number; count: number }
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
    node: V2GraphSimNode,
    highlightedTagId: string | null,
    simLinks: V2GraphSimLink[]
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
    const theme = readV2GraphThemeColors();

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
    const { categoryCells, simNodes, simLinks } = buildV2GraphSimulationData(data, width, height);

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
      fontSize: 9,
      fontWeight: '600',
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

    function redrawNode(node: V2GraphSimNode): void {
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

      const fillColor = node.done ? theme.peach : theme.paper;
      graphics.circle(0, 0, node.radius);
      graphics.fill({ color: hexToNum(fillColor), alpha: node.done ? 0.75 : 0.95 });
      graphics.stroke({ color: hexToNum(theme.ink), width: 1.6, alpha: 0.55 });
    }

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
        if (Math.abs(dx) > V2_GRAPH_TAP_THRESHOLD || Math.abs(dy) > V2_GRAPH_TAP_THRESHOLD) {
          didMove = true;
          hasUserTransformed = true;
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
          void onItemSelect(node.rawId);
        }
      };

      graphics.on('globalpointermove', onMove);
      graphics.on('pointerup', onUp);
      graphics.on('pointerupoutside', onUp);
    }

    simulation = forceSimulation(simNodes)
      .force(
        'link',
        forceLink<V2GraphSimNode, V2GraphSimLink>(simLinks)
          .id((node) => node.id)
          .distance((link) => (link.target.nodeType === 'item' ? 74 : 62))
          .strength(0.38)
      )
      .force(
        'charge',
        forceManyBody<V2GraphSimNode>().strength((node) =>
          node.nodeType === 'tag' ? -120 : -70
        )
      )
      .force('center', forceCenter(width / 2, height / 2).strength(0.04))
      .force('category', categoryForce(categoryCells))
      .force('tag', tagForce(simLinks))
      .force(
        'collide',
        forceCollide<V2GraphSimNode>().radius((node) =>
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

    function getCategoryMembraneRect(cell: V2GraphCategoryCell): CategoryMembraneRect | null {
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

    simulation.tick(90);
    fitWorldToContent();
    fitTimeoutIds = [80, 260, 560, 920].map((delay) => setTimeout(fitWorldToContent, delay));
    simulation.on('end', fitWorldToContent);

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

    function drawEdges(): void {
      edgeGraphics.clear();
      highlightEdgeGraphics.clear();

      for (const link of simLinks) {
        const sourceX = link.source.x ?? 0;
        const sourceY = link.source.y ?? 0;
        const targetX = link.target.x ?? 0;
        const targetY = link.target.y ?? 0;
        const isHighlighted =
          highlightedTagId &&
          (link.source.id === highlightedTagId || link.target.id === highlightedTagId);
        const graphics = isHighlighted ? highlightEdgeGraphics : edgeGraphics;
        graphics.moveTo(sourceX, sourceY);
        graphics.lineTo(targetX, targetY);
      }

      edgeGraphics.stroke({
        color: hexToNum(theme.inkMuted),
        width: 1.2,
        alpha: highlightedTagId ? 0.14 : 0.34
      });
      highlightEdgeGraphics.stroke({
        color: hexToNum(theme.skyStrong),
        width: 2.4,
        alpha: 0.8
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
            node.nodeType === 'tag' ? node.y ?? 0 : (node.y ?? 0) + node.radius + 4;
          node.textObj.alpha = connected ? 1 : 0.28;
        }
      }

      animFrameId = requestAnimationFrame(render);
    }

    render();

    let isPanning = false;
    let panPointerId: number | null = null;
    let panStart = { x: 0, y: 0 };
    let worldStart = { x: 0, y: 0 };
    const activePointers = new Set<number>();
    const canvas = app.canvas as HTMLCanvasElement;

    canvas.addEventListener('pointerdown', (event) => {
      activePointers.add(event.pointerId);
      if (isNodeDragging) return;
      if (activePointers.size === 1 && panPointerId === null) {
        isPanning = true;
        panPointerId = event.pointerId;
        panStart = { x: event.clientX, y: event.clientY };
        worldStart = { x: world.x, y: world.y };
      } else {
        isPanning = false;
        panPointerId = null;
      }
    });

    canvas.addEventListener('pointermove', (event) => {
      if (!isPanning || event.pointerId !== panPointerId || isNodeDragging) return;
      hasUserTransformed = true;
      world.x = worldStart.x + event.clientX - panStart.x;
      world.y = worldStart.y + event.clientY - panStart.y;
    });

    const stopPointer = (event: PointerEvent): void => {
      activePointers.delete(event.pointerId);
      if (event.pointerId === panPointerId) {
        isPanning = false;
        panPointerId = null;
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

<div bind:this={canvasContainer} class="v2-graph-canvas" aria-hidden="true"></div>

<style>
  .v2-graph-canvas {
    position: absolute;
    inset: 0;
    overflow: hidden;
    touch-action: none;
  }

  .v2-graph-canvas :global(canvas) {
    display: block;
  }
</style>
