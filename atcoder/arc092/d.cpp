#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI a(n), b(n);
  REP(i, 0, n) cin >> a[i];
  REP(i, 0, n) cin >> b[i];
  vector<vector<VI> > cb(29, vector<VI>(2, VI()));
  REP(i, 0, 29) {
    REP(j, 0, n) {
      int pat =  b[j] & ((1 << i) - 1);
      int pop = b[j] & 1 << i ? 1 : 0;
      cb[i][pop].push_back(pat);
    }
    REP(pop, 0, 2) {
      sort(cb[i][pop].begin(), cb[i][pop].end());
    }
  }
  ll ans = 0;
  REP(i, 0, 29) {
    int bpop = 0;
    REP(j, 0, n) {
      if (b[j] & 1 << i) {
	bpop += 1;
      }
    }
    if (0&&i <= 3) {
      DEBUGP(i);
      DEBUGP(bpop);
    }
    REP(j, 0, n) {
      int pat = a[j] & ((1 << i) - 1);
      int apop = a[j] & 1 << i ? 1 : 0;
      int num = apop ? n - bpop : bpop;
      REP(pop, 0, 2) {
	int idx = cb[i][pop].end() - lower_bound(cb[i][pop].begin(), cb[i][pop].end(), (1 << i) - pat);
	if(0&&i<=3){
	  DEBUGP(pop);
	  DEBUGP(idx);
	}
	if (pop == apop) {
	  num += idx;
	} else {
	  num -= idx;
	}
      }
      if (0&&i <= 3) {
	DEBUGP(j);
	DEBUGP(num);
      }
      if (num % 2 != 0) {
	ans ^= 1 << i;
      }
    }
  }
  cout << ans << endl;
}
