      SUBROUTINE RDSLOG (XNAME,X)
      IMPLICIT NONE

*     FORMAL_PARAMETERS:
      CHARACTER*(*) XNAME
      LOGICAL X

**    local variables ; dummy set
      DOUBLE PRECISION D(1),DM
      REAL             R(1),RM
      INTEGER          I(1),IM
      CHARACTER*80     C(1),CM
      LOGICAL          L(1),LM

*     other
      INTEGER IL,IS
      SAVE

*     read a single LOGICAL value (value of IL is arbitrary) !!
      IL = 0
      CALL RDDATA (5,'RDSLOG',0,0,' ',IS,XNAME,'L',D,R,I,C,L,
     $             1,1,1,1,1,IL,
     $             DM,RM,IM,CM,LM)

      X = L(1)

      RETURN
      END
