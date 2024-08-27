SELECT
  content,
  users.display_name as author,
  blog_posts.created_at
FROM
  blog_posts
INNER JOIN users on users.id = blog_posts.author
WHERE
  blog_posts.id = $1