/** @file Type definitions for modules that currently don't have typings on DefinitelyTyped.
 *
 * This file MUST NOT `export {}` for the modules to be visible to other files. */

// ===========================
// === Module declarations ===
// ===========================

declare module '*.svg' {
    const PATH: string
    export default PATH
}

declare module '*.png' {
    const PATH: string
    export default PATH
}

// Required because this is a build artifact, which does not exist on a clean repository.
declare module '*/build.json' {
    /** Build metadata generated by the build CLI. */
    export interface BuildInfo {
        commit: string
        version: string
        engineVersion: string
        name: string
    }

    const BUILD_INFO: BuildInfo
    export default BUILD_INFO
}

declare module '*/gui/config.yaml' {
    /** Content of the GUI config file. */
    interface Config {
        windowAppScopeName: string
        windowAppScopeConfigName: string
        windowAppScopeThemeName: string
        projectManagerEndpoint: string
        minimumSupportedVersion: string
        engineVersionSupported: string
        languageEditionSupported: string
    }

    const DATA: Config
    export default DATA
}

declare module '@eslint/js' {
    /** A set of configurations. */
    interface Config {
        rules: Record<string, unknown>
    }

    /** Preset configurations defined by ESLint. */
    interface EslintConfigs {
        all: Config
        recommended: Config
    }

    /** The default export of the module. */
    interface Default {
        configs: EslintConfigs
    }

    const DEFAULT: Default
    export default DEFAULT
}

declare module 'eslint-plugin-jsdoc' {
    const DEFAULT: unknown
    export default DEFAULT
}

declare module 'eslint-plugin-react' {
    /** An ESLint configuration. */
    interface Config {
        plugins: string[]
        rules: Record<string, number>
        parserOptions: object
    }

    // The names come from a third-party API and cannot be changed.
    /* eslint-disable @typescript-eslint/naming-convention */
    /** Configurations defined by this ESLint plugin. */
    interface Configs {
        recommended: Config
        all: Config
        'jsx-runtime': Config
    }

    /** Deprecated rules contained in this ESLint plugin. */
    interface DeprecatedRules {
        'jsx-sort-default-props': object
        'jsx-space-before-closing': object
    }
    /* eslint-enable @typescript-eslint/naming-convention */

    /** The default export of this ESLint plugin. */
    interface Default {
        rules: Record<string, object>
        configs: Configs
        deprecatedRules: DeprecatedRules
    }

    // The names come from a third-party API and cannot be changed.
    // eslint-disable-next-line no-restricted-syntax
    export const deprecatedRules: DeprecatedRules

    const DEFAULT: Default
    export default DEFAULT
}

declare module 'eslint-plugin-react-hooks' {
    /** An ESLint configuration. */
    interface Config {
        plugins: string[]
        rules: Record<string, string>
    }

    /** Configurations defined by this ESLint plugin. */
    interface Configs {
        recommended: Config
    }

    /** Rules defined by this ESLint plugin. */
    interface ReactHooksRules {
        // The names come from a third-party API and cannot be changed.
        /* eslint-disable @typescript-eslint/naming-convention */
        'rules-of-hooks': object
        'exhaustive-deps': object
        /* eslint-enable @typescript-eslint/naming-convention */
    }

    /** The default export of this ESLint plugin. */
    interface Default {
        configs: Configs
        rules: ReactHooksRules
    }

    // The names come from a third-party API and cannot be changed.
    /* eslint-disable no-restricted-syntax */
    export const configs: Configs
    export const rules: ReactHooksRules
    /* eslint-enable no-restricted-syntax */

    const DEFAULT: Default
    export default DEFAULT
}

declare module 'esbuild-plugin-time' {
    import * as esbuild from 'esbuild'

    export default function (name?: string): esbuild.Plugin
}

declare module 'tailwindcss/nesting/index.js' {
    import * as nested from 'postcss-nested'

    const DEFAULT: nested.Nested
    export default DEFAULT
}

declare module 'create-servers' {
    import * as http from 'node:http'

    /** Configuration options for `create-servers`. */
    interface CreateServersOptions {
        http: number
        handler: http.RequestListener
    }

    /** An error passed to a callback when a HTTP request fails. */
    interface HttpError {
        http: string
    }

    export default function (
        option: CreateServersOptions,
        // The types come from a third-party API and cannot be changed.
        // eslint-disable-next-line no-restricted-syntax
        errorHandler: (err: HttpError | undefined) => void
    ): unknown
}

declare module 'wasm_rust_glue' {
    const DEFAULT: unknown
    export default DEFAULT
}
