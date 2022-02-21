      INTEGER FUNCTION DTFSECMP (IYEAR1,IDOY1,IYEAR2,IDOY2)
      IMPLICIT NONE

*     FORMAL_PARAMETERS:
      INTEGER IYEAR1, IDOY1, IYEAR2, IDOY2

**    no local variables
      SAVE

      IF (IYEAR1.LT.1500.OR.IYEAR2.LT.1500.OR.IYEAR1.EQ.IYEAR2) THEN
         IF (IDOY2.LT.IDOY1) THEN
            DTFSECMP = -1
         ELSE IF (IDOY2.GT.IDOY1) THEN
            DTFSECMP = 1
         ELSE
            DTFSECMP = 0
         END IF
      ELSE IF (IYEAR2.LT.IYEAR1) THEN
         IF (IYEAR2.LT.IYEAR1) THEN
            DTFSECMP = -1
         ELSE IF (IYEAR2.GT.IYEAR1) THEN
            DTFSECMP = 1
         END IF
      END IF

      RETURN
      END
