module.exports = {
  title: 'BASIC言語を作ろう',
  author: '松本　幸大',
  language: 'ja',
  size: 'B5',
  theme: '@vivliostyle/theme-techbook', // .css or local dir or npm package. default to undefined.
  entryContext: './manuscripts',
  entry: [
    '0-preface.md', // `title` is automatically guessed from the file (frontmatter > first heading).
    '1-plan.md',
    // {
    //   path: 'epigraph.md',
    //   title: 'Epigraph', // title can be overwritten (entry > file),
    //   theme: '@vivliostyle/theme-whatever', // theme can be set indivisually. default to the root `theme`.
    // },
    // 'glossary.html', // html can be passed.
  ],
  toc: true, // whether generate and include toc.html or not (does not affect manifest.json), default to `true`. if `string` given, use it as a custom toc.html.
  // cover: './cover.png', // cover image. default to undefined.
  // workDir: './dist', // default to `.vivliostyle`.
}
