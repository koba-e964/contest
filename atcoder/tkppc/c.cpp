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
  int n, m, s;
  cin >> n >> m >> s;
  vector<PI> tk(n + 1);
  REP(i, 0, n) {
    cin >> tk[i].first >> tk[i].second;
  }
  tk[n] = PI(s, 0);
  tk.push_back(PI(0, 0));
  sort(tk.begin(), tk.end());
  int p = 0;
  int cur = 0;
  int curt = 0;
  REP(i, 0, tk.size()) {
    if (cur >= m) {
      p += tk[i].first - curt;
    }
    curt = tk[i].first;
    cur += tk[i].second;
  }
  cout << p << endl;
}
