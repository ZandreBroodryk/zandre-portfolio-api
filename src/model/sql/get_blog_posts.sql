SELECT
  id,
  title,
  created_at,
  CARDINALITY (STRING_TO_ARRAY (content, ' ')) as "word_count!"
FROM
  blog_posts
ORDER BY
  created_at DESC
LIMIT
  10