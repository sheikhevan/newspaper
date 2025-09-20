#set page(
  paper: "us-letter",
  margin: (x: 0.75in, y: 1in),
  header: [
    #set text(size: 10pt)
    #grid(
      columns: (1fr, 1fr, 1fr),
      align: (left, center, right),
      [*The Rustacean Times*],
      [{{ newspaper.date }}],
      [Page #context counter(page).display()],
    )
    #line(length: 100%)
  ],
)

#align(center)[
  #text(size: 24pt, weight: "bold")[The Rustacean Times]
  #v(0.3em)
  #text(size: 14pt)[{{ newspaper.date }}]
  #v(0.5em)
  #line(length: 60%)
]

#v(1em)

{% for article in newspaper.articles %}
#block(
  width: 100%,
  inset: 8pt,
  radius: 4pt,
  [
    #text(size: 14pt, weight: "bold")[{{ article.title }}]

    #text(size: 9pt, style: "italic")[
      By {{ article.author }} | {{ article.pub_date }}
    ]

    #v(0.5em)

    {{ article.content }}

    #v(0.3em)

    #text(size: 8pt, fill: blue)[
      #link("{{ article.link }}")[Read full article →]
    ]
  ],
)

#v(1em)
{% endfor %}

#align(center)[
  #text(size: 8pt, style: "italic")[
    Generated from {{ newspaper.total_articles }} RSS articles
  ]
]
