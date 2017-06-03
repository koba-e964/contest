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
  VI s(n);
  VI t, u;
  REP(i, 0, n) {
    cin >> s[i];
    if (s[i] % 10 == 0) {
      t.push_back(s[i]);
    } else {
      u.push_back(s[i]);
    }
  }
  if (u.size() == 0) {
    cout << 0 << endl;
    return 0;
  }
  int tot = 0;
  sort(u.begin(), u.end());
  REP(i, 0, u.size()) {
    tot += u[i];
  }
  REP(i, 0, t.size()) {
    tot += t[i];
  }
  if (tot % 10 == 0) {
    tot -= u[0];
  }
  cout << tot << endl;
}
