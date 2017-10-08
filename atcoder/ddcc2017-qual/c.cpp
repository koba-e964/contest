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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  int n;
  cin >> n;
  ll c;
  cin >> c;
  VL l(n);
  REP(i, 0, n) {
    cin >> l[i];
  }
  sort(l.rbegin(), l.rend());
  int start = 0;
  int end = n - 1;
  int cnt = 0;
  while (start <= end) {
    if (start == end) {
      cnt += 1;
      break;
    }
    if (l[start] + l[end] <= c - 1) {
      start += 1;
      end -= 1;
      cnt += 1;
      continue;
    }
    start += 1;
    cnt += 1;
  }
  cout << cnt << endl;
}
