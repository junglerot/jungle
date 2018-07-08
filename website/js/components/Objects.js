import { h } from 'hyperapp'
import Emoji from './Emoji'
import Code from './Code'
import Heading from './Heading'
const Subheading = Heading('Objects')
import TIP_COLLECTION from '../../snippets/tip-collection'
import TIP_INSTANCE from '../../snippets/tip-instance'
import ACCESS_TIPPY_INSTANCE from '../../snippets/access-tippy-instance'
import SHORTCUTS from '../../snippets/shortcuts'

export default () => (
  <section class="section">
    <Emoji class="section__icon-wrapper">🏷️</Emoji>
    <Heading>Objects</Heading>
    <p>
      When using Tippy.js, there are two types of objects to think about:
      collections and instances.
    </p>

    <Subheading>Collections</Subheading>
    <p>
      Whenever you call <code>tippy()</code>, you are potentially creating many
      tippys at once. It returns an object containing information about the
      tippys you created.
    </p>
    <Code content="const tipCollection = tippy('.btn')" />
    <p>
      <code>tipCollection</code> is a plain object.
    </p>
    <Code content={TIP_COLLECTION} />

    <Subheading>Tippy instances</Subheading>
    <p>
      Stored on reference elements via the <code>_tippy</code> property, and
      inside the <code>instances</code> array of the collection.
    </p>
    <Code content={ACCESS_TIPPY_INSTANCE} />

    <p>
      Alternatively, you can use the <code>tippy.one()</code> method to return
      the instance directly, because only a single tippy is created.
    </p>
    <Code content="const tip = tippy.one('.btn')" />
    <p>
      <code>tip</code> is also a plain object.
    </p>
    <Code content={TIP_INSTANCE} />

    <Subheading>Shortcuts</Subheading>
    <p>There are a couple of shortcuts available for accessing the instance.</p>
    <Code content={SHORTCUTS} />
  </section>
)
