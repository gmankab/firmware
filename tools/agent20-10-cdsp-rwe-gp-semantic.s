.text
.global cdsp_clear_gp
cdsp_clear_gp:
{ r2=##0
  memw(gp+##0xc028c014)=r2.new }
{ jumpr lr }
