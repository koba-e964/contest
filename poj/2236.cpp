#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;


const int N=1002;
int x[N],y[N],alive[N];
int n,d,p,q;
char c;
int disj[N],res[N][N],pos[N];
int root(int x){
  return x-disj[x]?disj[x]=root(disj[x]):x;
}
void unite(int x, int y){
  disj[root(x)]=root(y);
}
int main(void){
  scanf("%d%d",&n,&d);
  REP(i,0,n){
    scanf("%d%d",x+i,y+i);
    disj[i]=i;
  }
  REP(i,0,n)
    REP(j,0,n)
      if(i-j&&pow(x[i]-x[j],2)+pow(y[i]-y[j],2)<=d*d)
	res[i][pos[i]++]=j;
  while(scanf("%*[\n]%c%d",&c,&p)){
    if(c>80){//S
      scanf("%d",&q);
      puts(root(--p)==root(--q)?"SUCCESS":"FAIL");
    }else{//O
      alive[--p]=1;
      REP(j,0,pos[p]) {
	int v=res[p][j];
	if(alive[v]){
	  unite(p,v);
	}
      }
    }
  }
}
