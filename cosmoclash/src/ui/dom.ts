/** A small element builder for the menu screens. */
type Child = Node | string | null | undefined | false;
export interface Props {
  class?: string; id?: string; text?: string; role?: string; title?: string; hidden?: boolean;
  type?: string; item?: string; bind?: string; dir?: string; tabIndex?: number;
  aria?: Record<string, string>; data?: Record<string, string>; style?: string;
}
export function el<K extends keyof HTMLElementTagNameMap>(tag: K, props: Props = {}, ...children: Child[]): HTMLElementTagNameMap[K] {
  const node = document.createElement(tag);
  if (props.class) node.className = props.class;
  if (props.id) node.id = props.id;
  if (props.text !== undefined) node.textContent = props.text;
  if (props.role) node.setAttribute('role', props.role);
  if (props.title) node.title = props.title;
  if (props.hidden) node.hidden = true;
  if (props.dir) node.dir = props.dir;
  if (props.style) node.setAttribute('style', props.style);
  if (props.tabIndex !== undefined) node.tabIndex = props.tabIndex;
  if (props.type && node instanceof HTMLButtonElement) node.type = props.type as 'button';
  if (props.item) node.dataset.item = props.item;
  if (props.bind) node.dataset.bind = props.bind;
  for (const [key, value] of Object.entries(props.aria ?? {})) node.setAttribute('aria-' + key, value);
  for (const [key, value] of Object.entries(props.data ?? {})) node.dataset[key] = value;
  for (const child of children) if (child !== null && child !== undefined && child !== false) node.append(child);
  return node;
}
/** A menu button: one focus item of a screen. */
export function button(item: string, label: string, extra: Props = {}): HTMLButtonElement {
  return el('button', { type: 'button', class: 'cc-btn' + (extra.class ? ' ' + extra.class : ''), item, ...extra, text: undefined }, label);
}
