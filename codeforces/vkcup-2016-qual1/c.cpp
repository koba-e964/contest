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
  int n;
  cin >> n;
  vector<string> code(n);
  REP(i, 0, n) {
    cin >> code[i];
  }
  int mi = 13; // minimum distance between codes
  REP(i, 0, n) {
    REP(j, 0, i) {
      int d = 0;
      REP(k, 0, 6) {
	d += code[i][k] != code[j][k];
      }
      mi = min(mi, d);
    }
  }
  cout << (mi - 1) / 2 << endl;
}
