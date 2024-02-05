module.exports = {
    extends: [
        'eslint:recommended',
        'plugin:@typescript-eslint/recommended',
        'plugin:@typescript-eslint/recommended-requiring-type-checking',
        'plugin:@typescript-eslint/strict',
    ],
    parser: '@typescript-eslint/parser',
    plugins: ['jsdoc', '@typescript-eslint'],
    root: true,
    rules: {
        '@typescript-eslint/no-explicit-any': 'off',
        '@typescript-eslint/no-var-requires': 'off',
        '@typescript-eslint/no-this-alias': 'off',
        '@typescript-eslint/ban-ts-comment': 'off',
        '@typescript-eslint/no-empty-function': 'off',
        '@typescript-eslint/naming-convention': 'error',
        '@typescript-eslint/no-unnecessary-condition': 'error',
        '@typescript-eslint/restrict-template-expressions': [
            'error',
            {
                allowAny: true,
                allowBoolean: true,
            },
        ],
        'jsdoc/check-access': 'warn',
        'jsdoc/check-alignment': 'warn',
        'jsdoc/check-indentation': 'warn',
        'jsdoc/check-line-alignment': 'warn',
        'jsdoc/check-param-names': 'warn',
        'jsdoc/check-property-names': 'warn',
        'jsdoc/check-syntax': 'warn',
        'jsdoc/check-tag-names': 'warn',
        'jsdoc/check-types': 'warn',
        'jsdoc/check-values': 'warn',
        'jsdoc/empty-tags': 'warn',
        'jsdoc/implements-on-classes': 'warn',
        'jsdoc/no-bad-blocks': 'warn',
        'jsdoc/no-defaults': 'warn',
        'jsdoc/no-multi-asterisks': 'warn',
        'jsdoc/no-types': 'warn',
        'jsdoc/no-undefined-types': 'warn',
        'jsdoc/require-asterisk-prefix': 'warn',
        'jsdoc/require-description': 'warn',
        // Disabled because of a bug: https://github.com/gajus/eslint-plugin-jsdoc/issues/942
        // 'jsdoc/require-description-complete-sentence': 'warn,
        'jsdoc/require-file-overview': 'warn',
        'jsdoc/require-hyphen-before-param-description': 'warn',
        'jsdoc/require-jsdoc': 'warn',
        'jsdoc/require-param-description': 'warn',
        'jsdoc/require-param-name': 'warn',
        'jsdoc/require-param-type': 'warn',
        'jsdoc/require-property': 'warn',
        'jsdoc/require-property-description': 'warn',
        'jsdoc/require-property-name': 'warn',
        'jsdoc/require-property-type': 'warn',
        'jsdoc/require-returns-check': 'warn',
        'jsdoc/require-returns-description': 'warn',
        'jsdoc/require-throws': 'warn',
        'jsdoc/require-yields': 'warn',
        'jsdoc/require-yields-check': 'warn',
        'jsdoc/tag-lines': 'warn',
        'jsdoc/valid-types': 'warn',
    },
    overrides: [
        {
            files: ['*.ts', '*.tsx'],
            parserOptions: {
                project: ['./tsconfig.json'],
                tsconfigRootDir: __dirname,
            },
        },
    ],
}
