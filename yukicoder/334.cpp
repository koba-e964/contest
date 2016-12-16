#include <cassert>
#include <iostream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
const int N = 12;
int n;
int a[N];
int dp[1 << N];

bool is_kadomatsu(int x, int y, int z) {
  if (a[x] != a[y] && a[y] != a[z] && a[z] != a[x]) {
    return (a[x] < a[y] && a[y] > a[z])
      || (a[x] > a[y] && a[y] < a[z]);
  }
  return false;
}

int main(void){
  cin >> n;
  REP(i, 0, n) {
    cin >> a[i];
  }
  REP(bits, 0, 1 << n) {
    int all_stuck = true;
    REP(i, 0, n) {
      if ((bits & (1 << i)) == 0) { continue; }
      REP(j, i + 1, n) {
	if ((bits & (1 << j)) == 0) { continue; }
	REP(k, j + 1, n) {
	  if ((bits & (1 << k)) == 0) { continue; }
	  if (is_kadomatsu(i, j, k)) {
	    all_stuck &= dp[bits ^ 1 << i ^ 1 << j ^ 1 << k];
	  }
	}
      }
    }
    dp[bits] = not all_stuck;
  }
  if (dp[(1 << n) - 1]) {
    REP(i, 0, n) {
      REP(j, i + 1, n) {
	REP(k, j + 1, n) {
	  if (is_kadomatsu(i, j, k) && dp[((1 << n) - 1) ^ 1 << i ^ 1 << j ^ 1 << k] == 0) {
	    cout << i << " " << j << " " << k << endl;
	    return 0;
	  }
	}
      }
    }
    assert (0);
  }
  cout << -1 << endl;
}
