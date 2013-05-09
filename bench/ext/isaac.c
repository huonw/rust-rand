/*
  ------------------------------------------------------------------------------
  rand.c: By Bob Jenkins.  My random number generator, ISAAC.  Public Domain
  MODIFIED:
  960327: Creation (addition of randinit, really)
  970719: use context, not global variables, for internal state
  980324: make a portable version
  010626: Note this is public domain
  ------------------------------------------------------------------------------
*/
/*
  ------------------------------------------------------------------------------
  rand.h: definitions for a random number generator
  By Bob Jenkins, 1996, Public Domain
  MODIFIED:
  960327: Creation (addition of randinit, really)
  970719: use context, not global variables, for internal state
  980324: renamed seed to flag
  980605: recommend RANDSIZL=4 for noncryptography.
  010626: note this is public domain
  ------------------------------------------------------------------------------
*/

/*
  ------------------------------------------------------------------------------
  Standard definitions and types, Bob Jenkins
  ------------------------------------------------------------------------------
*/
#include<stdlib.h>
#ifndef STANDARD
# define STANDARD
# ifndef STDIO
#  include <stdio.h>
#  define STDIO
# endif
# ifndef STDDEF
#  include <stddef.h>
#  define STDDEF
# endif
#include<stdint.h>
typedef  uint64_t  ub8;
#define UB8MAXVAL 0xffffffffffffffffLL
#define UB8BITS 64
typedef   int64_t  sb8;
#define SB8MAXVAL 0x7fffffffffffffffLL
typedef  uint32_t  ub4;   /* unsigned 4-byte quantities */
#define UB4MAXVAL 0xffffffff
typedef   int32_t  sb4;
#define UB4BITS 32
#define SB4MAXVAL 0x7fffffff
typedef  uint16_t  ub2;
#define UB2MAXVAL 0xffff
#define UB2BITS 16
typedef   int16_t  sb2;
#define SB2MAXVAL 0x7fff
typedef  uint8_t ub1;
#define UB1MAXVAL 0xff
#define UB1BITS 8
typedef   int8_t sb1;   /* signed 1-byte quantities */
#define SB1MAXVAL 0x7f
typedef                 int  word;  /* fastest type available */

#define bis(target,mask)  ((target) |=  (mask))
#define bic(target,mask)  ((target) &= ~(mask))
#define bit(target,mask)  ((target) &   (mask))
#define TRUE  1
#define FALSE 0
#define SUCCESS 0  /* 1 on VAX */

#endif /* STANDARD */

#ifndef STANDARD
#include "standard.h"
#endif

#ifndef RAND
#define RAND
#define RANDSIZL   (8)  /* I recommend 8 for crypto,  for simulations */
#define RANDSIZ    (1<<RANDSIZL)

/* context of random number generator */
struct randctx
{
  ub4 randcnt;
  ub4 randrsl[RANDSIZ];
  ub4 randmem[RANDSIZ];
  ub4 randa;
  ub4 randb;
  ub4 randc;
};
typedef  struct randctx  randctx;

/*
  ------------------------------------------------------------------------------
  If (flag==TRUE), then use the contents of randrsl[0..RANDSIZ-1] as the seed.
  ------------------------------------------------------------------------------
*/
void randinit(randctx *r, word flag);

void isaac(randctx *r);


/*
  ------------------------------------------------------------------------------
  Call isaac_rand(/o_ randctx *r _o/) to retrieve a single 32-bit random value
  ------------------------------------------------------------------------------
*/
#define isaac_rand(r)                                               \
  (!(r)->randcnt-- ?                                                \
   (isaac(r), (r)->randcnt=RANDSIZ-1, (r)->randrsl[(r)->randcnt]) : \
   (r)->randrsl[(r)->randcnt])

#endif  /* RAND */




#define ind(mm,x)  ((mm)[(x>>2)&(RANDSIZ-1)])
#define rngstep(mix,a,b,mm,m,m2,r,x)                        \
  {                                                         \
    x = *m;                                                 \
    a = ((a^(mix)) + *(m2)) & 0xffffffff;                   \
    m2++;                                                   \
    *(m++) = y = (ind(mm,x) + a + b) & 0xffffffff;          \
    *(r++) = b = (ind(mm,y>>RANDSIZL) + x) & 0xffffffff;    \
  }


void     isaac(randctx *ctx)
{
  //print_state(ctx);
  register ub4 a,b,x,y,*m,*mm,*m2,*r,*mend;
  mm=ctx->randmem; r=ctx->randrsl;
  a = ctx->randa; b = (ctx->randb + (++ctx->randc)) & 0xffffffff;
  for (m = mm, mend = m2 = m+(RANDSIZ/2); m<mend; )
    {
      rngstep( a<<13, a, b, mm, m, m2, r, x);
      rngstep( (a&0xffffffff)>>6 , a, b, mm, m, m2, r, x);
      rngstep( a<<2 , a, b, mm, m, m2, r, x);
      rngstep( (a&0xffffffff)>>16, a, b, mm, m, m2, r, x);
    }
  for (m2 = mm; m2<mend; )
    {
      rngstep( a<<13, a, b, mm, m, m2, r, x);
      rngstep( (a&0xffffffff)>>6 , a, b, mm, m, m2, r, x);
      rngstep( a<<2 , a, b, mm, m, m2, r, x);
      rngstep( (a&0xffffffff)>>16, a, b, mm, m, m2, r, x);
    }
  ctx->randb = b; ctx->randa = a;
  //print_state(ctx);
}


#define mix(a,b,c,d,e,f,g,h)                    \
  {                                             \
    a^=b<<11; d+=a; b+=c;                       \
    b^=(c&0xffffffff)>>2;  e+=b; c+=d;          \
    c^=d<<8;  f+=c; d+=e;                       \
    d^=(e&0xffffffff)>>16; g+=d; e+=f;          \
    e^=f<<10; h+=e; f+=g;                       \
    f^=(g&0xffffffff)>>4;  a+=f; g+=h;          \
    g^=h<<8;  b+=g; h+=a;                       \
    h^=(a&0xffffffff)>>9;  c+=h; a+=b;          \
  }

/* if (flag==TRUE), then use the contents of randrsl[] to initialize mm[]. */
void randinit(randctx *ctx, word flag)
{
  word i;
  ub4 a,b,c,d,e,f,g,h;
  ub4 *m,*r;
  ctx->randa = ctx->randb = ctx->randc = 0;
  m=ctx->randmem;
  r=ctx->randrsl;
  a=b=c=d=e=f=g=h=0x9e3779b9;  /* the golden ratio */

  for (i=0; i<4; ++i)          /* scramble it */
    {
      mix(a,b,c,d,e,f,g,h);
    }
  if (flag)
    {
      /* initialize using the contents of r[] as the seed */
      for (i=0; i<RANDSIZ; i+=8)
        {
          a+=r[i  ]; b+=r[i+1]; c+=r[i+2]; d+=r[i+3];
          e+=r[i+4]; f+=r[i+5]; g+=r[i+6]; h+=r[i+7];
          mix(a,b,c,d,e,f,g,h);
          m[i  ]=a; m[i+1]=b; m[i+2]=c; m[i+3]=d;
          m[i+4]=e; m[i+5]=f; m[i+6]=g; m[i+7]=h;
        }
      /* do a second pass to make all of the seed affect all of m */
      for (i=0; i<RANDSIZ; i+=8)
        {
          a+=m[i  ]; b+=m[i+1]; c+=m[i+2]; d+=m[i+3];
          e+=m[i+4]; f+=m[i+5]; g+=m[i+6]; h+=m[i+7];
          mix(a,b,c,d,e,f,g,h);
          m[i  ]=a; m[i+1]=b; m[i+2]=c; m[i+3]=d;
          m[i+4]=e; m[i+5]=f; m[i+6]=g; m[i+7]=h;
        }
    }
  else
    {
      for (i=0; i<RANDSIZ; i+=8)
        {
          /* fill in mm[] with messy stuff */
          mix(a,b,c,d,e,f,g,h);
          m[i  ]=a; m[i+1]=b; m[i+2]=c; m[i+3]=d;
          m[i+4]=e; m[i+5]=f; m[i+6]=g; m[i+7]=h;
        }
    }

  isaac(ctx);            /* fill in the first set of results */
  ctx->randcnt=RANDSIZ;  /* prepare to use the first set of results */
}


int main() {
  unsigned a;
  randctx *ctx = calloc(sizeof *ctx, 1);
  randinit(ctx, 0);
  ub4 sum = 0;
  for (a = 0; a <100000000; a++) {
    sum += isaac_rand(ctx);
  }
  printf("%u\n", sum);

  return 0;
}
