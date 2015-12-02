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
typedef pair<int, int> PI;
const double EPS=1e-9;



int main(void){
  vector<int> m(5), w(5);
  REP(i, 0, 5) cin >> m[i];
  REP(i, 0, 5) cin >> w[i];
  int tot = 0;
  REP(i, 0, 5) {
    int x = (i + 1) * 500;
    tot += max(3 * x / 10, (x - m[i] * x / 250) - 50 * w[i]);
  }
  int hs,hu;
  cin >> hs >> hu;
  cout << tot + 100 * hs - 50 * hu << endl;
}
