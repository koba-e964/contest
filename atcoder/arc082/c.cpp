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
  VI a(n);
  VI freq(123456);
  REP(i, 0, n) {
    cin >> a[i];
    freq[a[i] + 1] += 1;
  }
  int ma = 0;
  REP(i, 1, 123455) {
    int tot = freq[i - 1] + freq[i] + freq[i + 1];
    ma = max(ma, tot);
  }
  cout << ma << endl;
}
