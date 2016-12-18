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



int main(void){
  int n;
  cin >> n;
  VI mi(n, 2e9);
  REP(b, 0, 10) {
    REP(c, 0, 2) {
      VI q;
      REP(i, 0, n) {
	if (((i >> b) & 1) == c) {
	  q.push_back(i);
	}
      }
      if (q.size() == 0) {
	continue;
      }
      printf("%lu\n", q.size());
      REP(i, 0, q.size()) {
	printf("%d%s", q[i] + 1, (i == q.size() - 1 ? "\n" : " "));
      }
      fflush(stdout);
      VI ans(n);
      REP(i, 0, n) {
	scanf("%d", &ans[i]);
      }
      REP(i, 0, n) {
	if (((i >> b) & 1) != c) {
	  mi[i] = min(mi[i], ans[i]);
	}
      }
    }
  }
  printf("%d\n", -1);
  REP(i, 0, n) {
    printf("%d%s", mi[i], i == n - 1 ? "\n" : " ");
  }
  fflush(stdout);
}
