SELECT
  X'hex string' || x'hex string2',
  N'national string',
  'character '' string',
  1+2-3/4*5%6
from A@B.C D, _E E, #F
where !(D.d = E.e) and D.d != E.e and D.d <> E.e and D.d < E.e and D.d > E.e;
