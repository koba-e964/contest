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
const int N = 2510;
PI m[N],s[N];

int main(void){
  int c, l;
  scanf("%d%d",&c,&l);
  REP(i,0,c){
    int x,y;
    scanf("%d%d",&x,&y);
    m[i]=PI(y,x);
  }
  REP(i,0,l){
    int su,cov;
    scanf("%d%d",&su,&cov);
    s[i]=PI(su,cov);
  }
  sort(m,m+c);
  sort(s,s+l);
  int sum = 0;
  REP(i,0,c){
    PI mima=m[i];
    int start = lower_bound(s,s+l,PI(mima.second,-1e9))-s;
    int end = lower_bound(s,s+l,PI(mima.first,1e9))-s;
    // pick one with the least sfp
    REP(i, start, end) {
      if(s[i].second >= 1) {
	sum++;
	s[i].second--;
	break;
      }
    }
  }
  cout << sum << endl;
}
