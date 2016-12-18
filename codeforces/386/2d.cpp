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
  int n, k, a, b;
  cin >> n >> k >> a >> b;
  if (a == 0 || b == 0) {
    if (n == k) {
      REP(i, 0, n) {
	cout << (a == 0 ? 'B' : 'G');
      }
      cout << endl;
      return 0;
    }
    cout << "NO" << endl;
    return 0;
  }
  int sw = 0;
  if (a < b) {
    swap(a, b);
    sw = 1;
  }
  int q = (a + b) / (b + 1);
  int r = q * (b + 1) - a;
  if (q > k) {
    cout << "NO" << endl;
    return 0;
  }
  string s;
  s.reserve(n);
  REP(i, 0, b + 1) {
    REP(j, 0, q - (i + r <= b ? 0 : 1)) {
      s += 'G';
    }
    if (i < b) {
      s += 'B';
    }
  }
  if (sw) {
    REP(i, 0, n) {
      if (s[i] == 'G') {
	s[i] = 'B';
      } else {
	s[i] = 'G';
      }
    }
  }
  cout << s << endl;
}
