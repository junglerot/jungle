import { h } from 'hyperapp'

import Section from '../components/Section'
import Emoji from '../components/Emoji'
import Heading from '../components/Heading'

const TITLE = 'Browser Support'

export default () => (state, actions) => (
  <Section title={TITLE} emoji="💻">
    <p>
      Tippy supports browsers with <code>requestAnimationFrame</code> and{' '}
      <code>MutationObserver</code> support: See{' '}
      <a
        href="https://caniuse.com/#feat=mutationobserver"
        target="_blank"
        rel="noopener"
      >
        caniuse data
      </a>
      .
    </p>
    <p>
      IE10 is only partially supported unless you polyfill{' '}
      <code>MutationObserver</code>, then it is fully supported.
    </p>
  </Section>
)
