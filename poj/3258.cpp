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

ll l;
int n, m;
const int N = 50100;
ll d[N];

// Reference: http://d.hatena.ne.jp/komiyam/20120212/1328974372
bool check(ll v) {
  ll acc = 0;
  int tot = 0;
  REP(i,0,n){
    if(d[i] <= l - v && d[i] - acc >= v) {
      //cerr << "d[i]=" << d[i] << ", " << acc << endl;
      tot++;
      acc = d[i];
    }
  }
  //cerr << "v=" << v << ", tot=" << tot << endl;
  return n-tot<=m;
}

int main(void){
  scanf("%lld%d%d",&l,&n,&m);
  REP(i,0,n){
    scanf("%lld",&d[i]);
  }
  sort(d, d + n);
  ll lo = 1;
  ll hi = l+1;
  while (hi - lo > 1) {
    ll mid = (hi + lo) / 2;
    if (check(mid)) {
      lo = mid;
    } else {
      hi = mid;
    }
  }
  REP(i, 1, l + 1) {
    check(i);
  }
  printf("%lld\n", lo);
}
