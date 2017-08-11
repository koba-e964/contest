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



int main(void) {
  int h, w;
  cin >> h >> w;
  if (h * w % 2 == 0) {
    // Manego
    cout << "Second" << endl;
    while (true) {
      int a, b, c;
      cin >> a >> b >> c;
      if (a == -1) {
	return 0;
      }
      cout << (h + 1 - a) << " " << (w + 1 - b) << " " << (1 - c) << endl;
    }
  }
  cout << "First" << endl;
  cout << (h + 1) / 2 << " " << (w + 1) / 2 << " " << 0 << endl;
  while (true) {
    int a, b, c;
    cin >> a >> b >> c;
    if (a == -1) {
      return 0;
    }
    cout << (h + 1 - a) << " " << (w + 1 - b) << " " << c << endl;
  }
}
