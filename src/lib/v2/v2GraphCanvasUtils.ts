import type { SimulationLinkDatum, SimulationNodeDatum } from 'd3-force';
import type { Graphics, Text } from 'pixi.js';
import type { V2Category, V2GraphData, V2Tag, V2TodoItem } from '../../types';

export type V2GraphNodeType = 'item' | 'tag';

export interface V2GraphThemeColors {
  paper: string;
  canvas: string;
  white: string;
  sky: string;
  skyStrong: string;
  mint: string;
  peach: string;
  ink: string;
  inkMuted: string;
  stroke: string;
}

export interface V2GraphSimNode extends SimulationNodeDatum {
  id: string;
  rawId: number;
  nodeType: V2GraphNodeType;
  label: string;
  categoryId: number | null;
  done: boolean;
  radius: number;
  item: V2TodoItem | null;
  tag: V2Tag | null;
  graphics: Graphics | null;
  textObj: Text | null;
}

export interface V2GraphSimLink extends SimulationLinkDatum<V2GraphSimNode> {
  source: V2GraphSimNode;
  target: V2GraphSimNode;
  tagId: number;
}

export interface V2GraphCategoryCell {
  category: V2Category;
  left: number;
  right: number;
  top: number;
  bottom: number;
  centerX: number;
  centerY: number;
  textObj: Text | null;
}

export const V2_GRAPH_TAP_THRESHOLD = 7;

function getCssVar(name: string): string {
  return getComputedStyle(document.documentElement).getPropertyValue(name).trim();
}

export function hexToNum(hex: string): number {
  return Number.parseInt(hex.replace('#', ''), 16);
}

export function truncateGraphLabel(label: string, maxLength: number): string {
  if (label.length <= maxLength) return label;
  return `${label.slice(0, Math.max(1, maxLength - 1))}…`;
}

export function readV2GraphThemeColors(): V2GraphThemeColors {
  return {
    paper: getCssVar('--color-paper') || '#f8f7f3',
    canvas: getCssVar('--color-canvas') || '#f2efe8',
    white: getCssVar('--color-white') || '#ffffff',
    sky: getCssVar('--color-accent-sky') || '#a8bddb',
    skyStrong: getCssVar('--color-accent-sky-strong') || '#8ea9cf',
    mint: getCssVar('--color-accent-mint') || '#bfd9c8',
    peach: getCssVar('--color-accent-peach') || '#e9c1ad',
    ink: getCssVar('--color-ink') || '#5b5852',
    inkMuted: getCssVar('--color-ink-muted') || '#7a776f',
    stroke: getCssVar('--color-stroke') || '#d8d2c5'
  };
}

function categoryLayouts(
  categories: V2Category[],
  width: number,
  height: number,
  itemCountByCategoryId: Map<number, number>
) {
  const isNarrow = width < 560;
  const horizontalPadding = isNarrow ? 48 : 96;
  const verticalPadding = isNarrow ? 72 : 96;
  const cellGap = isNarrow ? 42 : 34;
  const cellWidth = isNarrow ? Math.max(240, Math.min(320, width - horizontalPadding * 2)) : 320;
  const cellHeight = isNarrow ? 240 : 250;

  if (isNarrow) {
    let top = verticalPadding;
    const graphWidth = Math.max(width, cellWidth + horizontalPadding * 2);
    const left = (graphWidth - cellWidth) / 2;

    return new Map(
      categories.map((category) => {
        const itemCount = itemCountByCategoryId.get(category.id) ?? 1;
        const height = Math.max(cellHeight, Math.min(430, 190 + itemCount * 38));
        const layout = {
          left,
          right: left + cellWidth,
          top,
          bottom: top + height,
          centerX: left + cellWidth / 2,
          centerY: top + height / 2
        };
        top += height + cellGap;
        return [category.id, layout];
      })
    );
  }

  const columns = isNarrow ? 1 : Math.max(1, Math.ceil(Math.sqrt(categories.length)));
  const rows = Math.max(1, Math.ceil(categories.length / columns));
  const gridWidth = columns * cellWidth + (columns - 1) * cellGap;
  const gridHeight = rows * cellHeight + (rows - 1) * cellGap;
  const graphWidth = Math.max(width, gridWidth + horizontalPadding * 2);
  const graphHeight = Math.max(height, gridHeight + verticalPadding * 2);
  const startX = (graphWidth - gridWidth) / 2;
  const startY = (graphHeight - gridHeight) / 2;

  return new Map(
    categories.map((category, index) => {
      const column = index % columns;
      const row = Math.floor(index / columns);
      const left = startX + column * (cellWidth + cellGap);
      const top = startY + row * (cellHeight + cellGap);
      return [
        category.id,
        {
          left,
          right: left + cellWidth,
          top,
          bottom: top + cellHeight,
          centerX: left + cellWidth / 2,
          centerY: top + cellHeight / 2
        }
      ];
    })
  );
}

export function buildV2GraphSimulationData(
  data: V2GraphData,
  width: number,
  height: number
): {
  categoryCells: V2GraphCategoryCell[];
  simNodes: V2GraphSimNode[];
  simLinks: V2GraphSimLink[];
} {
  const categoriesWithItems = data.categories.filter((category) =>
    data.items.some((item) => item.category_id === category.id)
  );
  const itemCountByCategoryId = new Map<number, number>();
  for (const item of data.items) {
    itemCountByCategoryId.set(item.category_id, (itemCountByCategoryId.get(item.category_id) ?? 0) + 1);
  }
  const layouts = categoryLayouts(categoriesWithItems, width, height, itemCountByCategoryId);
  const categoryCells = categoriesWithItems.map((category) => {
    const layout = layouts.get(category.id) ?? {
      left: width / 2 - 160,
      right: width / 2 + 160,
      top: height / 2 - 125,
      bottom: height / 2 + 125,
      centerX: width / 2,
      centerY: height / 2
    };
    return {
      category,
      left: layout.left,
      right: layout.right,
      top: layout.top,
      bottom: layout.bottom,
      centerX: layout.centerX,
      centerY: layout.centerY,
      textObj: null
    };
  });

  const itemNodes: V2GraphSimNode[] = data.items.map((item, index) => {
    const center = layouts.get(item.category_id) ?? { centerX: width / 2, centerY: height / 2 };
    return {
      id: `item-${item.id}`,
      rawId: item.id,
      nodeType: 'item',
      label: item.text,
      categoryId: item.category_id,
      done: item.done,
      radius: item.done ? 10 : 12,
      item,
      tag: null,
      graphics: null,
      textObj: null,
      x: center.centerX + ((index % 5) - 2) * 24,
      y: center.centerY + (Math.floor(index / 5) - 1) * 24
    };
  });
  const itemNodeById = new Map(itemNodes.map((node) => [node.rawId, node]));

  const tagNodes: V2GraphSimNode[] = data.tags.map((tag) => {
    const connectedItemNodes = data.tag_edges
      .filter((edge) => edge.tag_id === tag.id)
      .map((edge) => itemNodeById.get(edge.item_id))
      .filter((node): node is V2GraphSimNode => node !== undefined);
    const average =
      connectedItemNodes.length > 0
        ? connectedItemNodes.reduce(
            (acc, node) => ({
              x: acc.x + (node.x ?? width / 2),
              y: acc.y + (node.y ?? height / 2)
            }),
            { x: 0, y: 0 }
          )
        : { x: width / 2, y: height / 2 };
    const labelRadius = Math.min(42, Math.max(20, tag.name.length * 4 + 14));

    return {
      id: `tag-${tag.id}`,
      rawId: tag.id,
      nodeType: 'tag',
      label: tag.name,
      categoryId: null,
      done: false,
      radius: labelRadius,
      item: null,
      tag,
      graphics: null,
      textObj: null,
      x:
        connectedItemNodes.length > 0
          ? average.x / connectedItemNodes.length
          : width / 2,
      y:
        connectedItemNodes.length > 0
          ? average.y / connectedItemNodes.length - 44
          : height / 2
    };
  });
  const tagNodeById = new Map(tagNodes.map((node) => [node.rawId, node]));

  const simLinks = data.tag_edges
    .map((edge) => {
      const source = tagNodeById.get(edge.tag_id);
      const target = itemNodeById.get(edge.item_id);
      if (!source || !target) return null;
      return { source, target, tagId: edge.tag_id };
    })
    .filter((link): link is V2GraphSimLink => link !== null);

  return {
    categoryCells,
    simNodes: [...itemNodes, ...tagNodes],
    simLinks
  };
}
