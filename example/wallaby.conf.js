module.exports = function () {
  return {
    files: [
      './**/*.ts',
      '!./**/*.spec.ts'
    ],

    tests: [
      './**/*.spec.ts'
    ],
    env: {
      type: 'node'
    },
    testFramework: 'mocha'
  }
}
