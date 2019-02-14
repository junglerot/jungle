import Defaults from './defaults'
import { matches } from './ponyfills'

const keys = Object.keys(Defaults)

/**
 * Determines if an element can receive focus
 * @param {Element|Object} el
 * @return {Boolean}
 */
export function canReceiveFocus(el) {
  return el instanceof Element
    ? matches.call(
        el,
        'a[href],area[href],button,details,input,textarea,select,iframe,[tabindex]',
      ) && !el.hasAttribute('disabled')
    : true
}

/**
 * Returns an object of optional props from data-tippy-* attributes
 * @param {Element|Object} reference
 * @return {Object}
 */
export function getDataAttributeOptions(reference) {
  return keys.reduce((acc, key) => {
    const valueAsString = (
      reference.getAttribute(`data-tippy-${key}`) || ''
    ).trim()

    if (!valueAsString) {
      return acc
    }

    if (key === 'content') {
      acc[key] = valueAsString
    } else {
      try {
        acc[key] = JSON.parse(valueAsString)
      } catch (e) {
        acc[key] = valueAsString
      }
    }

    return acc
  }, {})
}

/**
 * Polyfills the virtual reference (plain object) with Element.prototype props
 * Mutating because DOM elements are mutated, adds `_tippy` property
 * @param {Object} virtualReference
 */
export function polyfillElementPrototypeProperties(virtualReference) {
  const polyfills = {
    isVirtual: true,
    attributes: virtualReference.attributes || {},
    setAttribute(key, value) {
      virtualReference.attributes[key] = value
    },
    getAttribute(key) {
      return virtualReference.attributes[key]
    },
    removeAttribute(key) {
      delete virtualReference.attributes[key]
    },
    hasAttribute(key) {
      return key in virtualReference.attributes
    },
    addEventListener() {},
    removeEventListener() {},
    classList: {
      classNames: {},
      add(key) {
        virtualReference.classList.classNames[key] = true
      },
      remove(key) {
        delete virtualReference.classList.classNames[key]
      },
      contains(key) {
        return key in virtualReference.classList.classNames
      },
    },
  }

  for (const key in polyfills) {
    virtualReference[key] = polyfills[key]
  }
}
