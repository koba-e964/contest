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
  cin>>n;
  VI a(n);
  REP(i,0,n){
    cin>>a[i];
  }
  VI one(n + 1), two(n + 1);
  REP(i, 0, n) {
    one[i + 1] = one[i];
    two[i + 1] = two[i];
    if (a[i] == 1) {
      one[i + 1] += 1;
    } else {
      two[i + 1] += 1;
    }
  }
  VI left(n + 1), right(n + 1);
  REP(l, 0, n + 1) {
    int ma = 0;
    REP(i, 0, l + 1) {
      ma = max(ma, one[i] + two[l] - two[i]);
    }
    left[l] = ma;
  }
  REP(r, 0, n + 1) {
    int ma = 0;
    REP(i, r, n + 1) {
      ma = max(ma, two[n] - two[i] + one[i] - one[r]);
    }
    right[r] = ma;
  }
  int ma = 0;
  REP(i, 0, n + 1) {
    ma = max(ma, left[i] + right[i]);
  }
  cout << ma << "\n";
}
