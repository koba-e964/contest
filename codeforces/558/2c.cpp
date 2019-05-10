#include <algorithm>
#include <cstdio>
#include <map>
#include <set>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long ll;
typedef pair<ll,ll>PL;
typedef pair<PL,ll>line;

ll gcd(ll x,ll y){
  while(y!=0){
    ll r=x%y;x=y;y=r;
  }
  return x;
}

const int N=1001;
ll x[N],y[N];
set<line> t;

int main(void) {
  int n;
  scanf("%d",&n);
  REP(i,0,n)scanf("%lld%lld",x+i,y+i);
  REP(i,0,n)REP(j,0,i){
    ll dx=x[j]-x[i],dy=y[j]-y[i];
    ll g=gcd(abs(dx),abs(dy));
    dx/=g,dy/=g;
    if(dy<0){
      dx=-dx,dy=-dy;
    }
    if(dy==0&&dx<0)dx=-dx;
    line l(PL(dy,-dx),dy*x[i]-dx*y[i]);
    t.insert(l);
  }
  ll sz=t.size();
  map<PL,ll>grad;
  for(line e:t){
    grad[e.first]+=1;
  }
  ll ans=sz*sz;
  for(auto e:grad)ans-=e.second*e.second;
  printf("%lld\n",ans/2);
}
