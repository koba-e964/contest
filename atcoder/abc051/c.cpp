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
  int sx, sy, tx, ty;
  cin >>sx>>sy>>tx>>ty;
  int dx = tx - sx;
  int dy = ty - sy;
  REP(i, 0, dx) cout << "R";
  REP(i, 0, dy) cout << "U";
  REP(i, 0, dx) cout << "L";
  REP(i, 0, dy) cout << "D";
  cout << "D";
  REP(i, 0, dx + 1) cout << "R";
  REP(i, 0, dy + 1) cout << "U";
  cout << "LU";
  REP(i, 0, dx + 1) cout << "L";
  REP(i, 0, dy + 1) cout << "D";
  cout << "R" << endl;
}
