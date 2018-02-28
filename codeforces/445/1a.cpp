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
  VI t(n + 1, -1);
  int cnt = 1;
  VI mas(n + 1, -1);
  VI ans(n + 1, -1);
  REP(i, 0, n) {
    cin >> t[i + 1];
  }
  ans[0] = 0;
  mas[0] = 0;
  REP(i, 0, n) {
    int prev = t[i + 1];
    int num = -1;
    if (mas[ans[prev]] == prev) {
      num = ans[prev];
    } else {
      num = cnt;
      cnt++;
    }
    ans[i + 1] = num;
    mas[num] = i + 1;
  }
  cout << cnt << endl;
}
