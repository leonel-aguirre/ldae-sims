INSERT INTO
  programs (
    program_code,
    display_name,
    program_type,
    duration_type,
    duration
  )
VALUES
  (
    'isc-2018-p',
    'Ingeniería en Sistemas Computacionales',
    'Licenciatura',
    'Semestral',
    9
  ),
  (
    'iin-2016-p',
    'Ingeniería Industrial',
    'Licenciatura',
    'Semestral',
    9
  ),
  (
    'ige-2017-p',
    'Ingeniería en Gestión Empresarial',
    'Licenciatura',
    'Semestral',
    9
  ),
  (
    'mcc-2018-p',
    'Ciencias de la Computación',
    'Maestría',
    'Semestral',
    4
  );

INSERT INTO
  specializations (specialization_code, display_name, program_code)
VALUES
  (
    'dae-2020-s',
    'Desarrollo de Aplicaciones Empresariales',
    'isc-2018-p'
  ),
  (
    'rct-2021-s',
    'Redes Convergentes de Alta Disponibilidad con Tecnologías Emergentes',
    'isc-2018-p'
  ),
  (
    'imc-2020-s',
    'Ingeniería en Manufactura y Calidad',
    'iin-2016-p'
  ),
  (
    'map-2020-s',
    'Manufactura en Artículos de Piel',
    'iin-2016-p'
  ),
  (
    'cip-2017-s',
    'Gestión de la Calidad e Innovación de Procesos',
    'ige-2017-p'
  );

INSERT INTO
  courses (
    course_code,
    display_name,
    program_code,
    specialization_code
  )
VALUES
  (
    'cdf-2014-c',
    'Cálculo Diferencial',
    'isc-2018-p',
    NULL
  ),
  (
    'fpg-2015-c',
    'Fundamentos de Programación',
    'isc-2018-p',
    NULL
  ),
  (
    'aln-2017-c',
    'Álgebra Lineal',
    'isc-2018-p',
    NULL
  ),
  (
    'pet-2012-c',
    'Probabilidad y Estadística',
    'isc-2018-p',
    NULL
  ),
  (
    'eda-2014-c',
    'Estructura de Datos',
    'isc-2018-p',
    NULL
  ),
  (
    'cvt-2016-c',
    'Cálculo Vectorial',
    'isc-2018-p',
    NULL
  ),
  (
    'fbd-2012-c',
    'Fundamentos de Base de Datos',
    'isc-2018-p',
    NULL
  ),
  (
    'tap-2018-c',
    'Tópicos Avanzados de Programación',
    'isc-2018-p',
    NULL
  ),
  (
    'cdt-2020-c',
    'Ciencia de Datos',
    'isc-2018-p',
    'dae-2020-s'
  ),
  (
    'dae-2020-c',
    'Desarrollo de Aplicaciones Empresariales',
    'isc-2018-p',
    'dae-2020-s'
  ),
  (
    'lae-2020-c',
    'Laboratorio para Despliegue de Aplicaciones Empresariales',
    'isc-2018-p',
    'dae-2020-s'
  ),
  (
    'drd-2021-c',
    'Diseño de Redes',
    'isc-2018-p',
    'rct-2021-s'
  ),
  (
    'srd-2021-c',
    'Seguridad en Redes',
    'isc-2018-p',
    'rct-2021-s'
  ),
  (
    'ida-2021-c',
    'Infraestructura para Despliegue de Aplicaciones',
    'isc-2018-p',
    'rct-2021-s'
  ),
  (
    'qmc-2010-c',
    'Química',
    'iin-2016-p',
    NULL
  ),
  (
    'din-2014-c',
    'Dibujo Industrial',
    'iin-2016-p',
    NULL
  ),
  (
    'mnm-2008-c',
    'Metrología y Normalización',
    'iin-2016-p',
    NULL
  ),
  (
    'tld-2015-c',
    'Taller de Liderazgo',
    'iin-2016-p',
    NULL
  ),
  (
    'ei1-2010-c',
    'Estadística Inferencial I',
    'iin-2016-p',
    NULL
  ),
  (
    'apy-2011-c',
    'Administración de Proyectos',
    'iin-2016-p',
    NULL
  ),
  (
    'pfb-2012-c',
    'Procesos de Fabricación',
    'iin-2016-p',
    NULL
  ),
  (
    'hsi-2015-c',
    'Higiene y Seguridad Industrial',
    'iin-2016-p',
    NULL
  ),
  (
    'rin-2020-c',
    'Robótica Industrial',
    'iin-2016-p',
    'imc-2020-s'
  ),
  (
    'clp-2020-c',
    'Controladores Lógicos Programables',
    'iin-2016-p',
    'imc-2020-s'
  ),
  (
    'dac-2019-c',
    'Diseño Asistido por Computadora',
    'iin-2016-p',
    'imc-2020-s'
  ),
  (
    'dmd-2019-c',
    'Diseño y Modelado',
    'iin-2016-p',
    'map-2020-s'
  ),
  (
    'tsm-2020-c',
    'Temas Selectos de Manufactura',
    'iin-2016-p',
    'map-2020-s'
  ),
  (
    'spc-2020-c',
    'Administración de los Sistemas de Producción del Calzado',
    'iin-2016-p',
    'map-2020-s'
  ),
  (
    'fin-2013-c',
    'Fundamentos de Investigación',
    'ige-2017-p',
    NULL
  ),
  (
    'dhu-2015-c',
    'Desarrollo Humano',
    'ige-2017-p',
    NULL
  ),
  (
    'sae-2017-c',
    'Software de Aplicación Ejecutivo',
    'ige-2017-p',
    NULL
  ),
  (
    'lab-2014-c',
    'Legislación Laboral',
    'ige-2017-p',
    NULL
  ),
  (
    'cem-2012-c',
    'Costos Empresariales',
    'ige-2017-p',
    NULL
  ),
  (
    'eem-2015-c',
    'Economía Empresarial',
    'ige-2017-p',
    NULL
  ),
  (
    'ipe-2017-c',
    'Instrumentos de Presupuestación Empresarial',
    'ige-2017-p',
    NULL
  ),
  (
    'ema-2016-c',
    'Entorno Macroeconómico',
    'ige-2017-p',
    NULL
  ),
  (
    'sca-2017-c',
    'Seminario de Calidad',
    'ige-2017-p',
    'cip-2017-s'
  ),
  (
    'gco-2016-c',
    'Gestión del Conocimiento',
    'ige-2017-p',
    'cip-2017-s'
  ),
  (
    'sco-2017-c',
    'Seminario de Consultoría Organizacional',
    'ige-2017-p',
    'cip-2017-s'
  ),
  (
    'mdi-2015-c',
    'Matemáticas Discretas',
    'mcc-2018-p',
    NULL
  ),
  (
    'si1-2016-c',
    'Seminario de Investigación I',
    'mcc-2018-p',
    NULL
  ),
  (
    'tco-2018-c',
    'Teoría de la Computación',
    'mcc-2018-p',
    NULL
  ),
  (
    'tes-2012-c',
    'Tesis',
    'mcc-2018-p',
    NULL
  );