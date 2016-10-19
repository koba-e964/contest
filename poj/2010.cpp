#include <algorithm>
#include <cstdio>
#include <queue>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

const int N=1<<17;
int n,c,f,x,y;
PI a[N];
int t[N],u[N];

int main(void){
  scanf("%d%d%d",&n,&c,&f);
  n/=2;
  REP(i,0,c){
    scanf("%d%d",&x,&y);
    a[i]=PI(x,y);
  }
  sort(a,a+c);
  priority_queue<int> pq;
  ll tot=0;
  REP(i,0,c){
    y=a[i].second;
    pq.push(y);
    tot+=y;
    if(i>=n){
      x=pq.top();pq.pop();
      tot-=x;
    }
    t[i]=tot;
  }
  pq=priority_queue<int>();
  tot=0;
  REP(i,0,c){
    y=a[c-1-i].second;
    pq.push(y);
    tot+=y;
    if(i>=n){
      x=pq.top();pq.pop();
      tot-=x;
    }
    u[c-i-1]=tot;
  }
  y=-1;
  REP(x,n,c-n){
    int sum=t[x-1]+a[x].second+u[x+1];
    if(sum<=f){
      y=x;
    }
  }
  printf("%d\n",y!=-1?a[y].first:-1);
}
