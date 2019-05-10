#include <algorithm>
#include <stdio.h>
#include <string>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long ll;

int main(void){
  ll n;
  scanf("%lld",&n);
  string s;
  while(n!=0){
    s+=n%2?'1':'0';
    n-=n%2?1:0;
    n/=-2;
  }
  if(!s.size())s="0";
  reverse(s.begin(),s.end());
  printf("%s\n",s.c_str());
}
