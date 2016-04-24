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
  int n, m, a, b;
  cin >> n >> m >> a >> b;
  int cur = n;
  REP(i, 0, m) {
    if (cur <= a) {
      cur += b;
    }
    
    int c;
    cin >> c;

    cur -= c;
    if (cur < 0) {
      cout << i + 1 << endl;
      return 0;
    }
  }
  cout << "complete" << endl;
}
