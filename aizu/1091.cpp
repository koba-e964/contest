#include <stdio.h>
#include <math.h>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;

int main(void){
  int a,l,x;
  for(;scanf("%d%d%d",&a,&l,&x)>0;){
    double h=sqrt(l*l-a*a/4.0);
    double u=sqrt((l+x)*(l+x)-l*l)/2.0;
    printf("%.15f\n",a*h/2+l*u);
  }
}
