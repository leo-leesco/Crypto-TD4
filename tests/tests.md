# Tests for basic Montgomery arithmetic and the Montgomery ladder

Check the following equalities *on the level of (X,Z)-coordinates*.
This means you should *ignore the Y-coordinates* (which are included
below for completeness).

Don't forget that we are using projective coordinates.  When checking
your answers, (x:y:1) = (Lx:Ly:L) for any nonzero L in the base field.

## Tests with small p and variable A

These are simple tests that you can run if p and A are variable in
your code.   If you've hard-coded in the parameters from Curve25519,
then you'll need the other tests further below.

### First curve:

prime: $p = 101$

curve constant: $A = 49$ in GF(p) (so $(A+2)/4 = 38 mod 101$)

curve: $E: Y^2*Z = X*(X^2 + A*X*Z + Z^2)$

base point: P = (X:Y:Z) = (2:2:1)

Check these (on (X,Z)-coordinates only):

* [2]P = (70:81:1)
* [3]P = (59:61:1)
* [77]P = (8:90:1)


### Second curve:

prime: $p = 1009$

curve constant: $A = 682$ in GF(p) (so $(A+2)/4 = 171 mod 1009$)

curve: $E: Y^2*Z = X*(X^2 + A*X*Z + Z^2)$

base point: P = (X:Y:Z) = (7:207:1)

Check these (on (X,Z)-coordinates only):

* [2]P = (284:3:1)
* [3]P = (759:824:1)
* [5]P = (1000:308:1)
* [34]P = (286:675:1)
* [104]P = (810:312:1)
* [947]P = (755:481:1)



## Tests for Curve25519

prime: $p = 2^255 - 19$

curve constant: $A = 486662$ (so $(A+2)/4 = 121666$)

curve: $E: Y^2*Z = X*(X^2 + A*X*Z + Z^2)$

base point: P = (X:Y:Z) = (9:*:1)
where y = 14781619447589544791020593568409986887264606134616475288964881837755586237401

Check the following on (X,Z)-coordinates only.
Note that [3]P = [2]P + P (difference P), [4]P = [2][2]P and [3]P + P (difference [2]P), etc., so you can use these to test your xADD and xDBL functions, as well as your LADDER.


* [2]P = (x_2:y_2:1) where
    * x_2 is 14847277145635483483963372537557091634710985132825781088887140890597596352251
    * y_2 is 8914613091229147831277935472048643066880067899251840418855181793938505594211
* [3]P = (x_3:y_3:1) where
    * x_3 is 12697861248284385512127539163427099897745340918349830473877503196793995869202
    * y_3 is 18782504731206017997790968374142055202547214238579664877619644464800823583275
* [4]P = (x_4:y_4:1) where
    * x_4 is 55094879196667521951171181671895976763495004283458921215716618814842818532335
    * y_4 is 3326902261410125624348978312040953107588569168976051259550493007501242276912
* [5]P = (x_5:y_5:1) where 
    * x_5 is 29723531761959712214579609737676588517305008794118309711793522224007834336391
    * y_5 is 40874084639449471205709022392342867673774078808580340264992287101339353033704
* [7]P = (x_7:y_7:1) where
    * x_7 is 6189616607995615193367150877376005513902989163470402290395604116858034460712
    * y_7 is 50374912218183152874782770359920331956288684995949584070970333666982233842018
