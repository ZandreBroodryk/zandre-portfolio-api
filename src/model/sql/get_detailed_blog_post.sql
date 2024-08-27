SELECT
  content,
  author,
  created_at
FROM
  blog_posts
WHERE
  id = $1