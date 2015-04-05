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
const int E = 250000;

int n,m;
int x[502];
int a[E], b[E], c[E];
int main(void){
	scanf("%d%d", &n, &m);
	REP(i, 0, n) {
		scanf("%d", x + i);
	}
	REP(i, 0, m) {
		scanf("%d%d%d", a+i, b+i, c+i);
		a[i]--;
		b[i]--;
	}
	double ma = 0;
	REP(i,0,m) {
		double t = (double)(x[a[i]] + x[b[i]]) / (double)c[i];
		ma = max(ma, t);
	}
	printf("%.10f\n", ma);
}
