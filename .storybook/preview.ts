import type { Preview } from '@storybook/sveltekit';

import '../src/app.css';

const preview: Preview = {
  parameters: {
    layout: 'fullscreen',
    backgrounds: {
      default: 'Tickly Paper',
      values: [
        { name: 'Tickly Paper', value: '#f8f7f3' },
        { name: 'Tickly Canvas', value: '#f2efe8' },
        { name: 'Dark', value: '#1f2937' }
      ]
    },
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i
      }
    }
  }
};

export default preview;
