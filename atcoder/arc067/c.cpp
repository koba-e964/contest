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
  int  n;
  cin >> n;
  vector<bool> pr(n + 1, true);
  REP(i, 2, n + 1) {
    REP(j, 2, n / i + 1) {
      pr[i * j] = false;
    }
  }
  ll sum = 1;
  REP(p, 2, n + 1) {
    if (pr[p]) {
      // count p
      int v = n;
      int cnt = 0;
      while (v > 0) {
	v /= p;
	cnt += v;
      }
      sum *= cnt + 1;
      sum %= mod;
    }
  }
  cout << sum << endl;
}
