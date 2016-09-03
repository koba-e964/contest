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
  int a = 0;
  int b = 0;
  REP(loop_cnt, 0, n) {
    int m, c;
    cin >> m >> c;
    if (m < c) {
      b++;
    }
    if (m > c) {
      a++;
    }
  }
  if (a < b) {
    cout << "Chris" << endl;
  }
  if (a > b) {
    cout << "Mishka" << endl;
  }
  if (a == b) {
    cout << "Friendship is magic!^^" << endl;
  }
}
