import browser, { isBrowser } from './browser'
import defaults from './defaults'
import selectors from './selectors'

/**
 * Injects a string of CSS styles to the style node in the document head
 */
export const injectCSS = css => {
  if (isBrowser && browser.isSupported) {
    const head = document.head || document.querySelector('head')
    const style = document.createElement('style')
    style.type = 'text/css'
    head.insertBefore(style, head.firstChild)

    if (style.styleSheet) {
      style.styleSheet.cssText = css
    } else {
      style.appendChild(document.createTextNode(css))
    }
  }
}

/**
 * Ponyfill for Array.from; converts iterable values to an array
 */
export const toArray = value => [].slice.call(value)

/**
 * Sets an attribute on an element; aids in minification
 */
export const setAttr = (el, attr, value = '') => {
  el.setAttribute(attr, value)
}

/**
 * Sets the content of a tooltip
 */
export const setContent = (content, options) => {
  content[options.allowTitleHTML ? 'innerHTML' : 'textContent'] = options.title
}

/**
 * Applies a transition duration to a list of elements
 */
export const applyTransitionDuration = (els, value) => {
  els.filter(Boolean).forEach(el => {
    // A slightly faster backdrop transition looks better...
    const v = matches.call(el, selectors.BACKDROP)
      ? Math.round(value / 1.2)
      : value
    el.style[prefix('transitionDuration')] = `${v}ms`
  })
}

/**
 * Returns the inner elements of a popper element
 */
export const getInnerElements = popper => {
  const select = s => popper.querySelector(s)
  return {
    tooltip: select(selectors.TOOLTIP),
    backdrop: select(selectors.BACKDROP),
    content: select(selectors.CONTENT),
    arrow: select(selectors.ARROW) || select(selectors.ROUND_ARROW)
  }
}

/**
 * Determines if a value is a virtual reference (aka plain object)
 */
export const isVirtualReference = value =>
  ({}.toString.call(value) === '[object Object]')

/**
 * Creates and returns a div element
 */
export const div = () => document.createElement('div')

/**
 * Sets the innerHTML of an element while tricking linters & minifiers (for FF extensions)
 */
export const setInnerHTML = (el, html) => {
  const o = { x: true }
  el[o.x && 'innerHTML'] =
    html instanceof Element ? html[o.x && 'innerHTML'] : html
}

/**
 * Returns an array of elements based on the value
 */
export const getArrayOfElements = value => {
  if (value instanceof Element || isVirtualReference(value)) return [value]
  if (value instanceof NodeList) return toArray(value)
  if (Array.isArray(value)) return value

  try {
    return toArray(document.querySelectorAll(value))
  } catch (e) {
    return []
  }
}

/**
 * Determines if a value is numeric
 */
export const isNumeric = value => !isNaN(value) && !isNaN(parseFloat(value))

/**
 * Returns a value at a given index depending on if it's an array or number
 */
export const getValue = (value, index) =>
  Array.isArray(value) ? value[index] : value

/**
 * Constructs the popper element and returns it
 */
export const createPopperElement = (id, reference, options) => {
  const popper = div()
  popper.className = 'tippy-popper'
  popper.role = 'tooltip'
  popper.id = `tippy-${id}`
  popper.style.zIndex = options.zIndex
  popper.style.maxWidth = options.maxWidth

  const tooltip = div()
  tooltip.className = 'tippy-tooltip'
  setAttr(tooltip, 'data-size', options.size)
  setAttr(tooltip, 'data-animation', options.animation)
  setAttr(tooltip, 'data-state', 'hidden')
  options.theme.split(' ').forEach(t => {
    tooltip.classList.add(t + '-theme')
  })

  const content = div()
  content.className = 'tippy-content'

  if (options.interactive) {
    setAttr(tooltip, 'data-interactive')
  }

  if (options.arrow) {
    const arrow = div()
    if (options.arrowType === 'round') {
      arrow.className = 'tippy-roundarrow'
      arrow.innerHTML =
        '<svg viewBox="0 0 24 8" xmlns="http://www.w3.org/2000/svg"><path d="M3 8s2.021-.015 5.253-4.218C9.584 2.051 10.797 1.007 12 1c1.203-.007 2.416 1.035 3.761 2.782C19.012 8.005 21 8 21 8H3z"/></svg>'
    } else {
      arrow.className = 'tippy-arrow'
    }
    tooltip.appendChild(arrow)
  }

  if (options.animateFill) {
    const backdrop = div()
    backdrop.className = 'tippy-backdrop'
    setAttr(backdrop, 'data-state', 'hidden')
    setAttr(tooltip, 'data-animatefill')
    tooltip.appendChild(backdrop)
  }

  if (options.inertia) {
    setAttr(tooltip, 'data-inertia')
  }

  if (options.html) {
    const { html } = options
    const isTemplateString = typeof html === 'string' && html[0] === '<'
    const isElement = html instanceof Element

    if (isTemplateString) {
      setInnerHTML(content, html)
    } else if (isElement) {
      content.appendChild(html)
    } else {
      try {
        setInnerHTML(content, document.querySelector(html))
      } catch (e) {}
    }
  } else {
    content[options.allowTitleHTML ? 'innerHTML' : 'textContent'] =
      options.title
  }

  if (options.interactive) {
    setAttr(popper, 'tabindex', '-1')
  }

  tooltip.appendChild(content)
  popper.appendChild(tooltip)

  return popper
}

/**
 * Hides all visible poppers on the document
 */
export const hideAllPoppers = excludeTippy => {
  toArray(document.querySelectorAll(selectors.POPPER)).forEach(popper => {
    const tippy = popper._tippy
    if (!tippy) return

    const { options } = tippy

    if (
      (options.hideOnClick === true || options.trigger.indexOf('focus') > -1) &&
      (!excludeTippy || popper !== excludeTippy.popper)
    ) {
      tippy.hide()
    }
  })
}

/**
 * Adds event listeners to the reference based on the trigger option
 */
export const addEventListeners = (
  reference,
  options,
  { onTrigger, onMouseLeave, onBlur, onDelegateShow, onDelegateHide }
) => {
  const listeners = []

  const on = (eventType, handler) => {
    reference.addEventListener(eventType, handler)
    listeners.push({ eventType, handler })
  }

  const oppositeEvents = {
    mouseenter: 'mouseleave',
    mouseover: 'mouseout',
    focus: 'blur',
    focusin: 'focusout'
  }

  return options.trigger
    .trim()
    .split(' ')
    .reduce((acc, eventType) => {
      if (eventType === 'manual') {
        return acc
      }

      if (!options.target) {
        on(eventType, onTrigger)
        switch (eventType) {
          case 'mouseenter':
            on('mouseleave', onMouseLeave)
          case 'focus':
            on(browser.isIE ? 'focusout' : 'blur', onBlur)
        }
      } else {
        switch (eventType) {
          case 'mouseenter':
            on('mouseover', onDelegateShow)
            on('mouseout', onDelegateHide)
          case 'focus':
            on('focusin', onDelegateShow)
            on('focusout', onDelegateHide)
          case 'click':
            on(eventType, onDelegateShow)
        }
      }

      return acc.concat(listeners)
    }, [])
}

const defaultsKeys = Object.keys(defaults)
/**
 * Returns an object of options from data-tippy-* attributes
 */
export const getDataAttributeOptions = reference =>
  defaultsKeys.reduce((acc, key) => {
    const valueAsString = reference.getAttribute(`data-tippy-${key}`)

    if (!valueAsString) {
      return acc
    }

    if (valueAsString === 'true') {
      acc[key] = true
    } else if (valueAsString === 'false') {
      acc[key] = false
    } else if (isNumeric(valueAsString)) {
      acc[key] = Number(valueAsString)
    } else if (key !== 'target' && valueAsString.trim()[0] === '[') {
      acc[key] = JSON.parse(valueAsString)
    } else {
      acc[key] = valueAsString
    }

    return acc
  }, {})

/**
 * Polyfills the virtual reference (plain object) with needed props
 */
export const polyfillVirtualReferenceProps = virtualReference => {
  const reference = {
    isVirtual: true,
    attributes: virtualReference.attributes || {},
    setAttribute: (key, value) => {
      out.attributes[key] = value
    },
    getAttribute: key => virtualReference.attributes[key],
    removeAttribute: key => {
      delete virtualReference.attributes[key]
    },
    addEventListener: () => {},
    removeEventListener: () => {},
    classList: {
      classNames: {},
      add: key => {
        reference.classList.classNames[key] = true
      },
      remove: key => {
        delete reference.classList.classNames[key]
      },
      contains: key => !!reference.classList.classNames[key]
    }
  }

  return reference
}

/**
 * Ponyfill for Element.prototype.matches
 */
export const matches = (() => {
  if (isBrowser) {
    const e = Element.prototype
    return (
      e.matches ||
      e.matchesSelector ||
      e.webkitMatchesSelector ||
      e.mozMatchesSelector ||
      e.msMatchesSelector ||
      function(s) {
        const matches = (this.document || this.ownerDocument).querySelectorAll(
          s
        )
        let i = matches.length
        while (--i >= 0 && matches.item(i) !== this) {} // eslint-disable-line no-empty
        return i > -1
      }
    )
  }
})()

/**
 * Ponyfill for Element.prototype.closest
 */
export const closest = (element, parentSelector) => {
  const fn =
    Element.prototype.closest ||
    function(selector) {
      let el = this
      while (el) {
        if (matches.call(el, selector)) {
          return el
        }
        el = el.parentElement
      }
    }

  return fn.call(element, parentSelector)
}

/**
 * Focuses an element while preventing a scroll jump if it's not within the viewport
 */
export const focus = el => {
  const x = window.scrollX || window.pageXOffset
  const y = window.scrollY || window.pageYOffset
  el.focus()
  scroll(x, y)
}

/**
 * Resets a popper's position to fix Popper.js#251
 */
export const resetPopperPosition = popper => {
  Object.assign(popper.style, {
    top: '',
    left: '',
    [prefix('transform')]: ''
  })
}

/**
 * Triggers reflow
 */
export const reflow = popper => {
  void popper.offsetHeight
}

/**
 * Transforms the x/y axis ased on the placement
 */
export const transformAxisBasedOnPlacement = (axis, isVertical) => {
  if (!axis) return ''
  return isVertical
    ? axis
    : {
        X: 'Y',
        Y: 'X'
      }[axis]
}

/**
 * Transforms the scale/translate numbers based on the placement
 */
export const transformNumbersBasedOnPlacement = (
  type,
  numbers,
  isVertical,
  isReverse
) => {
  if (!numbers.length) return ''

  const transforms = {
    scale: (() => {
      if (numbers.length === 1) {
        return `${numbers[0]}`
      } else {
        return isVertical
          ? `${numbers[0]}, ${numbers[1]}`
          : `${numbers[1]}, ${numbers[0]}`
      }
    })(),
    translate: (() => {
      if (numbers.length === 1) {
        return isReverse ? `${-numbers[0]}px` : `${numbers[0]}px`
      } else {
        if (isVertical) {
          return isReverse
            ? `${numbers[0]}px, ${-numbers[1]}px`
            : `${numbers[0]}px, ${numbers[1]}px`
        } else {
          return isReverse
            ? `${-numbers[1]}px, ${numbers[0]}px`
            : `${numbers[1]}px, ${numbers[0]}px`
        }
      }
    })()
  }

  return transforms[type]
}

/**
 * Computes the arrow's transform so that it is correct for any placement
 */
export const computeArrowTransform = (popper, arrow, arrowTransform) => {
  const placement = getPopperPlacement(popper)
  const isVertical = placement === 'top' || placement === 'bottom'
  const isReverse = placement === 'right' || placement === 'bottom'

  const getAxis = re => {
    const match = arrowTransform.match(re)
    return match ? match[1] : ''
  }

  const getNumbers = re => {
    const match = arrowTransform.match(re)
    return match ? match[1].split(',').map(parseFloat) : []
  }

  const re = {
    translate: /translateX?Y?\(([^)]+)\)/,
    scale: /scaleX?Y?\(([^)]+)\)/
  }

  const matches = {
    translate: {
      axis: getAxis(/translate([XY])/),
      numbers: getNumbers(re.translate)
    },
    scale: {
      axis: getAxis(/scale([XY])/),
      numbers: getNumbers(re.scale)
    }
  }

  const computedTransform = arrowTransform
    .replace(
      re.translate,
      `translate${transformAxisBasedOnPlacement(
        matches.translate.axis,
        isVertical
      )}(${transformNumbersBasedOnPlacement(
        'translate',
        matches.translate.numbers,
        isVertical,
        isReverse
      )})`
    )
    .replace(
      re.scale,
      `scale${transformAxisBasedOnPlacement(
        matches.scale.axis,
        isVertical
      )}(${transformNumbersBasedOnPlacement(
        'scale',
        matches.scale.numbers,
        isVertical,
        isReverse
      )})`
    )

  arrow.style[prefix('transform')] = computedTransform
}

/**
 * Sets the visibility state of a popper so it can begin to transition in or out
 */
export const setVisibilityState = (els, type) => {
  els.filter(Boolean).forEach(el => {
    el.setAttribute('data-state', type)
  })
}

/**
 * Prefixes a CSS property with the one supported by the browser
 */
export const prefix = property => {
  const prefixes = ['', 'webkit']
  const upperProp = property[0].toUpperCase() + property.slice(1)

  for (let i = 0; i < prefixes.length; i++) {
    const prefix = prefixes[i]
    const prefixedProp = prefix ? prefix + upperProp : property
    if (typeof document.body.style[prefixedProp] !== 'undefined') {
      return prefixedProp
    }
  }

  return null
}

/**
 * Update's a popper's position and runs a callback onUpdate; wrapper for async updates
 */
export const updatePopperPosition = (
  popperInstance,
  callback,
  updateAlreadyCalled
) => {
  const { popper, options } = popperInstance
  const onCreate = options.onCreate
  const onUpdate = options.onUpdate

  options.onCreate = options.onUpdate = () => {
    reflow(popper)
    callback && callback()
    onUpdate()
    options.onCreate = onCreate
    options.onUpdate = onUpdate
  }

  if (!updateAlreadyCalled) {
    popperInstance.scheduleUpdate()
  }
}

/**
 * Defers a function's execution until the next animation frame and until
 * the call stack has cleared
 */
export const defer = fn => {
  requestAnimationFrame(() => {
    setTimeout(fn, 1)
  })
}

/**
 * Determines if the mouse cursor is outside of the popper's interactive border
 * region
 */
export const cursorIsOutsideInteractiveBorder = (event, popper, options) => {
  if (!popper.getAttribute('x-placement')) return true

  const { clientX: x, clientY: y } = event
  const { interactiveBorder, distance } = options

  const rect = popper.getBoundingClientRect()
  const placement = getPopperPlacement(popper)
  const borderWithDistance = interactiveBorder + distance

  const exceeds = {
    top: rect.top - y > interactiveBorder,
    bottom: y - rect.bottom > interactiveBorder,
    left: rect.left - x > interactiveBorder,
    right: x - rect.right > interactiveBorder
  }

  switch (placement) {
    case 'top':
      exceeds.top = rect.top - y > borderWithDistance
      break
    case 'bottom':
      exceeds.bottom = y - rect.bottom > borderWithDistance
      break
    case 'left':
      exceeds.left = rect.left - x > borderWithDistance
      break
    case 'right':
      exceeds.right = x - rect.right > borderWithDistance
      break
  }

  return exceeds.top || exceeds.bottom || exceeds.left || exceeds.right
}

/**
 * Returns the distance offset, taking into account the default offset due to
 * the transform: translate() rule in CSS
 */
export const getOffsetDistanceInPx = distance =>
  -(distance - defaults.distance) + 'px'

/**
 * Returns the popper's placement, ignoring shifting (top-start, etc)
 */
export const getPopperPlacement = popper =>
  popper.getAttribute('x-placement').split('-')[0]

/**
 * Evaluates an object of options
 */
export const evaluateOptions = (reference, options) => {
  const out = {}

  if (options.arrow) {
    out.animateFill = false
  }

  if (typeof options.appendTo === 'function') {
    out.appendTo = options.appendTo(reference)
  }

  if (typeof options.html === 'function') {
    out.html = options.html(reference)
  }

  return { ...options, ...out }
}
