#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;

const int N = 100001;
ll x;
ll a[N], b[N];
ll c[N];
int n,d;

void swap(ll *ary, int a, int b) {
	ll tmp = ary[a];
	ary[a] = ary[b];
	ary[b] = tmp;
}

//x is 64-bit variable;
ll getNextX() {
    x = (x * 37 + 10007) % 1000000007;
    return x;
}
void initAB() {
	int i;
    for(i = 0; i < n; i = i + 1){
        a[i] = i + 1;
    }
    for(i = 0; i < n; i = i + 1){
        swap(a, i, getNextX() % (i + 1));
    }
    for(i = 0; i < n; i = i + 1){
        if (i < d)
            b[i] = 1;
        else
            b[i] = 0;
    }
    for(i = 0; i < n; i = i + 1){
        swap(b, i, getNextX() % (i + 1));
    }
}
int main(void){
	ios::sync_with_stdio(false);
	cin >> n >> d >> x;
	initAB();
	REP(i, 0, n) {
		ll ma = 0;
		REP(j, 0, i+1) {
			ma = max(ma, a[j] * b[i-j]);
		}
		c[i] = ma;
	}
	REP(i, 0, n) {
		cout << c[i] << "\n";
	}
}
