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

// A
int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n; cin >> n;
  string t;
  REP(i, 0, n) {
    string s;
    cin >> s;
    sort(s.begin(), s.end());
    t += s[0];
  }
  sort(t.begin(), t.end());
  cout << t << endl;
}
