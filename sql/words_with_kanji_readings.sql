SELECT dict.id, dict.sequence as sequence, dict.reading, dict.kanji, dict.no_kanji, dict.priorities, dict.information, dict.kanji_info, dict.jlpt_lvl, dict.is_main
FROM dict JOIN
  Dict AS D2 ON D2.sequence = dict.sequence
WHERE 
  d2.reading LIKE $1 AND dict.reading LIKE $2 AND (dict.is_main = $3 OR d2.is_main = $3)
ORDER BY
  LENGTH(d2.reading), d2.priorities