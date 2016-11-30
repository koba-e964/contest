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

PI ask(int a, int b, int c, int d) {
  set<int> test;
  test.insert(a);
  test.insert(b);
  test.insert(c);
  test.insert(d);
  assert (test.size() == 4);
  cout << a << " " << b << " " << c << " " << d << endl;
  int x, y;
  cin >> x >> y;
  if (x == 4 && y == 0) {
    exit(0);
  }
  return PI(x, y);
}

int main(void){
  int mi = 10;
  VI t(10, 7);
  REP(i, 3, 10) {
    PI res = ask(0, 1, 2, i);
    int r = res.first + res.second;
    mi = min(mi, r);
    t[i] = r;
  }
  REP(i, 3, 10) {
    t[i] -= mi;
  }
  mi = 10;
  REP(i, 0, 3) {
    PI res = ask(i, 7, 8, 9);
    int r = res.first + res.second;
    mi = min(mi, r);
    t[i] = r;
  }
  REP(i, 0, 3) {
    t[i] -= mi;
  }
  VI s;
  REP(i, 0, 10) {
    if (t[i] == 1) {
      s.push_back(i);
    }
  }
  assert (s.size() == 4);
  do {
    ask(s[0], s[1], s[2], s[3]);
  } while (next_permutation(s.begin(), s.end()));
}
