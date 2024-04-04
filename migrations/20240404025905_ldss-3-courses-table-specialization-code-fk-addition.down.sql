ALTER TABLE
  courses DROP CONSTRAINT IF EXISTS courses_specialization_code_fkey;

ALTER TABLE
  courses DROP COLUMN IF EXISTS specialization_code;