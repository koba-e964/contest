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
  int x[3], y[3];
  REP(i, 0, 3) {
    cin >> x[i] >> y[i];
  }
  cout << 3 << endl;
  int ca[3] = {1, -1, 1}, cb[3] = {1, 1, -1};
  REP(i, 0, 3) {
    cout << ca[i] * (x[1] - x[0]) + cb[i] * (x[2] - x[0]) + x[0];
    cout << " ";
    cout << ca[i] * (y[1] - y[0]) + cb[i] * (y[2] - y[0]) + y[0];
    cout << endl;
  }
}
