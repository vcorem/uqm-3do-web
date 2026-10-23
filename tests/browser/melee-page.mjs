// The Melee Remastered page is the HD page with the variant that
// scripts/package-editions.py writes into /melee/index.html. A test that runs an
// unpackaged HD build gets the same variant before any page script runs; a
// packaged page sets its own (its engine label and save folder) over it.
export const MELEE_VARIANT = Object.freeze({ engine: 'c', saveDir: '/home/web_user/.uqm-melee', meleeOnly: true });

export function meleePage(html, variant = MELEE_VARIANT) {
  if (html.split('<head>').length !== 2) throw new Error('Expected one <head> in the HD page');
  return html.replace('<head>', '<head>\n<script>window.UQM_VARIANT = ' + JSON.stringify(variant) + ';</script>');
}

export async function useMeleePage(context, variant = MELEE_VARIANT) {
  await context.addInitScript(value => { window.UQM_VARIANT = value; }, { ...variant });
}
