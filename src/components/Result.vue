<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { Application, FileType } from '../type';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { invoke } from '@tauri-apps/api';

const selectIndex = ref(0);

const props = defineProps({
  list: Array<Application>,
  directive: String,
  searchval: String,
});

const list = computed(() => {
  return props.list;
})

const searchval = computed(() => {
  return props.searchval;
})

const directive = computed(() => {
  return props.directive;
});

onMounted(() => {
  document.addEventListener('keydown', handleKeyDown);
})

const fileToUrl = (filePath: string, r_type: FileType) => {
  if(filePath)return convertFileSrc(filePath)

  switch (r_type) {
    case 'PDF':
      // 根据文件类型返回相应的 Base64 图片
      return "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAAACXBIWXMAAAsTAAALEwEAmpwYAAACFklEQVR4nO3XS0sbURQH8HyPgmgKFSoFHxFtU5T6LNT4oKaGtm4kolFjLQWFQOxCoyIKKlaopVZDW0mcsVB3hRYXthk/haskagZHJ9FMEpO/zBVEKGJG72RceOAs5i7u+XHPvQdGp7uNmxpgN5vAcMtgfN50MjbuZg87HHcoArgwWA7pZsz1CSGjKbJn7s2iBYCSjI99AV9cidDj+kPB+i4n84DxJQjlJnoIKAQkJtw4qHlODwGlgEk3wnUt9BBQCDie8SBssvyPKG+KHPW+z1IdAMaHmHMKUfsgonYnjmwORKz9EFv7INqcW9QB0aF5CJYexGfZtIB0ASs+BHJLEbhXgtCT5swDUit/EbhrwI6hBtuFVRqcAMth12iCP6cAfLVFG0DkjQv+7HwctA1oA0jM/SAA4UWXNoCwfQh+fQFpgzTyWZs7ILR0QzDbELz/CIkPq6cX9Ns6kl//qAuQxhbJ8cenPYhPe8mTlBHBPCNZl3P34TMk3b/VAfDVFgT0RdgxVJFi2/kV4GtfkXb49YXkW16XXAt0Acfza9hrtJLNgw/KyAs4Pwljk9+x3/oW/NOXENsdZF7QATD/IFodZPjIxUMVZqS8G2ndfioAsXPwrLfy+E18/Kmo+LUB+6/7zopLw5c/OeqAlGcD0ugCkou/rlRclTkA1QEMJ1IDMJyoHLDKNSj5Mbk4uWWwvnrFgNvQZShOAAhzR6U81UfDAAAAAElFTkSuQmCC";
    case 'Folder':
      // 根据文件类型返回相应的 Base64 图片
      return "data:image/png;base64,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";
    case 'TXT':
      // 根据文件类型返回相应的 Base64 图片
      return "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAAACXBIWXMAAAsTAAALEwEAmpwYAAAB/0lEQVR4nO3XS08aURwFcDZN/A4mRNe1VTe+aOOioumCmesHsLquMenLRxBXbNxQJEYjaHRpQmKMWmPbhQu0QNNuKCIMk4hBBqUpvtAE4+M0lyZNKWGYwRl0wU3O7s6cX/5zZ5LRaMrrvi5GQCcbxzwrwCkl7d9SC00rvyoVA7ACUiQOSM0z1zFqLdyZziloFQEQGeU0ba5jPBz2U8T5U+d+9R0ATlAz7FcOQYoAPDJuKYcgMgF61wkeGwPKIYhMQIcnhVpjIAdRZw2f6ZYPtaoDSAxoXfyJltm9TJpno2ia+5MnSwcR9QFx8dwa0LKxg8Z1XlLoXsUBFW8n8eDVuKTQvf9ea4ik4WW6ez2GHn3RAFJkaLltO4ofNseml+1OS0aQ/25UNf0Z2qm1gqlf9WeVW7Z48DyPcDh85Ruzb9JJlARgiKQx4TEhsKHD91AIHMeBCwZNJXkEhkgaU+4BJNc1mVBEaPvroOTyWx3CPivejD3/W05j97wrzWvY8CmA0Q8vc8rpRFQHNIiUq/8d6LPi9fuOvOWKAIjIgaOnPbu8P6tcNQCzewGHt3C5aoAuLomj4FDBcnUAwg3MX3xIJBI4Cg5ixj2Qt1wVABu9xMhHF8xuH15wyczjENsvHyDgVBQQu84g6CQKTwunxQBYOT8meRPHPBMDIxtQXpoSrd9iUtxD38T8YwAAAABJRU5ErkJggg==";
    case 'LNK':
      // 根据文件类型返回相应的 Base64 图片
      return "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAAACXBIWXMAAAsTAAALEwEAmpwYAAACwElEQVR4nO2V3WoTQRTHT0Qpbayt+FGcVRDri0Sv+gBiEaU3PoDQ3ewsba0FtV8gqK1RMitaFHorCuYFlJY2Ns1MGqsVO9sLaYJXLRik7shu0/1Immo2CYj0D3O1h/2d8z9n5gDs63/SMZJvRXHeIxHjEdL5S0nnY9LT1QsgRKjhcESMbknneUk3ROlBOp89HTfONxIuS4Sbu8Hdw/NnnvHORsCVMhgxMojwhER4zucE4TN1bQcqgSPdKEj62tWd7x1T38ISMab9ya1FGgnvKo3bTsJ1AunGSD3g8t/A3Xie8MRO1Qrv9g7cn+CWJGJk3RbwicDw5tEsQvHVDc9QbbWPZfoAKg+WRFav+YfUnZHqJM9fabk5t+n9WcvAnIDeOQGYzgJmZffccmbbIaf69RMT64erh2MqgzJvto9QB94xsSLgxnsBmG0fjW2CQiN7wE2J8EvVw1Wq2AAlKY6OMyeB4/eXv0PvfN5JwDoq3QAQB8rgdrsMOTgcWwksiLY7qS1PRbnw+MpJwOydN4nmW1RH8a+lcKU2OLaqWywcGV7sL3nxppsGs2dBZb924sJDH0z05EuNcExlHxzTAkQzXdYzai0WX3WT2fyh/vRPx4HBlECPP9cAj9JuwNQsgxdlbTXv1jv1gIqDfdRJtmVoQaDYJzNYz5VsK2CarwTfkbXVrMWCYsui7W7S4xQTbbeTP9Dkx8sQSJj1+PuervxoCBFqGkg/B82ND2nUbBrm5yCwVBbzXKnMnrHRpQH/nNiO3QsOt6TSF54fJqAauJZ+DTVLZaOuAywHvalweQztB+wOXdGtt1AXaZmL/h+zaV8SKr0OmJkltr+B+kmEiovFm0TObgdmS2W2161yr6yt5ruKux1qgpp+BQ2TTDtBpTMVElgHnAmw1aqWCIFGI4DpiH07VPrQfhcGWYB9vq9/WL8Bcg7fR0FD7BQAAAAASUVORK5CYII=";
    case 'JPG':
      // 根据文件类型返回相应的 Base64 图片
      return "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAAACXBIWXMAAAsTAAALEwEAmpwYAAACB0lEQVR4nO3T/0sTcRzH8f2eWWbf/L7m1rBSlH7wK0aE0g+B+6V/o3+ifsjQQouQqU1tMHV+qUiCtIK+YEQFg36obc5t7u62u+XWbXO6nC/xstiFcrfts/vFveH523HvB2/uVKr85Oe/MdAY6qKwaaABku2+c1AlNV05WJ6KkAQYcrT8bzkDXGe2MBoKY/4Xh1tcTHnAG54F1l3/urkPImeAWNwtAuxcQlGAI0qJAKbVsLKAG4EN2KOUcIlXPCt8E0QBnd8TqJ8PoWEhhKuO38r+BVe+rUM/w0I3FRDSP2HR+SOhDOCyLQ6d1Q/thLiz0wHhKqnPXvMkhYgB2j/HUD3OQGPZO63Vj45dxKWva6h78RO1c0G0f1nLHtD2KQq1mZZMM86g8R2P+perfwDPgzj/lEPrx2jmgObFCCrHKNlVPaZwYS4oAtTMsmhZjKQPuPg6jHKTL+0qRnw494wTAfRTATR/iKQHKBteQaZVPPKhZoYVAXQTfjS95+UDSoxeZFPp0IqwOBWgtTBofMvLA5wa8CDbThu90E36RQCNmRYuIQk48dANEp0c8KDawogA6lFKGlB8fxmkOv7AjTNmOj1AUZ8LJDvWvwz1GCUfcPTuEkhXdM+FKpNPHqCwx5Es7HGCdEd6l1A+7E1KAg51240Ft+2bBd0OkOzwHSeK+1w2SUB+DtxsA0B9iY9TrVvKAAAAAElFTkSuQmCC";
    case 'PNG':
      // 根据文件类型返回相应的 Base64 图片
      return "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAAACXBIWXMAAAsTAAALEwEAmpwYAAACB0lEQVR4nO3T/0sTcRzH8f2eWWbf/L7m1rBSlH7wK0aE0g+B+6V/o3+ifsjQQouQqU1tMHV+qUiCtIK+YEQFg36obc5t7u62u+XWbXO6nC/xstiFcrfts/vFveH523HvB2/uVKr85Oe/MdAY6qKwaaABku2+c1AlNV05WJ6KkAQYcrT8bzkDXGe2MBoKY/4Xh1tcTHnAG54F1l3/urkPImeAWNwtAuxcQlGAI0qJAKbVsLKAG4EN2KOUcIlXPCt8E0QBnd8TqJ8PoWEhhKuO38r+BVe+rUM/w0I3FRDSP2HR+SOhDOCyLQ6d1Q/thLiz0wHhKqnPXvMkhYgB2j/HUD3OQGPZO63Vj45dxKWva6h78RO1c0G0f1nLHtD2KQq1mZZMM86g8R2P+perfwDPgzj/lEPrx2jmgObFCCrHKNlVPaZwYS4oAtTMsmhZjKQPuPg6jHKTL+0qRnw494wTAfRTATR/iKQHKBteQaZVPPKhZoYVAXQTfjS95+UDSoxeZFPp0IqwOBWgtTBofMvLA5wa8CDbThu90E36RQCNmRYuIQk48dANEp0c8KDawogA6lFKGlB8fxmkOv7AjTNmOj1AUZ8LJDvWvwz1GCUfcPTuEkhXdM+FKpNPHqCwx5Es7HGCdEd6l1A+7E1KAg51240Ft+2bBd0OkOzwHSeK+1w2SUB+DtxsA0B9iY9TrVvKAAAAAElFTkSuQmCC";
    case 'GIF':
      // 根据文件类型返回相应的 Base64 图片
      return "data:image/png;base64,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";
    case 'XLSX':
      // 根据文件类型返回相应的 Base64 图片
      return "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAAACXBIWXMAAAsTAAALEwEAmpwYAAACIUlEQVR4nGNgGAWjgAwgPNlPirfPx5e7072Bu911M0ez0zMGmoD99iziCyK0ReeGxYnMDOkQnhq0WWiS3xuBCb7/+Xq9//N0efznbnf7z9ni/J9qdiqsSsqVXR4/V2ZJ3DmphTE/JedH/hefE/5fdGbIf5FpQf+FJvv/x+YAg8MeoQTxUTcrwg5YnfxfbkXCf9mlcf+lF8X8J9YBhkfcCePDbj8H1gFH3P8PXwfAAMiS3W8OwfnI4iQ5QHF54v/ltw/833T/xP/8Q9PhDvBcU/F//a0j/5dc2f1fqNeHdg6QXRr3v/XsCrDm9z8+/1eYF/NfdFrw/9PPb4DF6g7No30UKCxJ+H/34zOwhbXHFvxP2tkDZt9+9+S/6IQAFAeAfA3CIEt6782E85HFyUoDcXu6wZY+/PTy//lXd8Bsv7XVtE8DskiJcO/jC3CD1t46hDURlt1oAWOQJXEX8+F8ZHGyHCA1P+r/waeX4A7Y8/AsfR1QcGQG2OJjz67+f/jxJZgds7mNPlGguzLz/9vvn8AGBG1p/J+/fyqY/eTT6/+Sk4Jpnwg33j8OtnD/k4vgckB8esj/m+8eg8X6T62hbTa03FD0f/ODk+CCyHl9ObwgCt3YCC6IVl3f/19iQiB9coH4QBTFskOtMjIcEg44TEyDZBXtmmSGR9wtGQZ9o5Qg+M/AyNfrq8Lf6x3K3eXRxt3mto2jxfk5YY2jYBQwYAUAZK+9R5jV2g4AAAAASUVORK5CYII=";
    case 'PPT':
      // 根据文件类型返回相应的 Base64 图片
      return "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAAACXBIWXMAAAsTAAALEwEAmpwYAAADa0lEQVR4nO2W208UVxzHp1Fjgol9K16ABQQXhssM2QoKyhZkZwdZXGbMmgg1mj6s2Z3Fl75qwVvapOFS/wGjiS/7YrwBKiyXZXdduSiz7dLYxmgblFaWJkRjxOrXzLY0s+wsmZnw1PBNvs+fz+/MOWcOQaxlLRoz47SlzQnMoZhgvTznYcSYx/rHm/YWPzrdv6PTHUSX8C263BUA8QmxmoHTtCEmMM6YYJ2NeayQ901biwRGQjuFB+hw1awKfP5kXdacx/pwOXhFgf/qvoi2tvX64R6GUppavUC8d/BD60bN8F+/tHw242KevHBZoNSXAqNWQOqVFWFeB7HuQSN5Pmwr8IYaCryheqM3eLD4ZaCxGKO2Yow2FMF/gMSIVJbEMEsiwJvw7ESdWgHpc3yVUiDcSLJhWyHuNxQgdMCIUL0RQXYnAkw+Ri158NftwMj+XIzU5GL4ixwMmbMxaM7G5DEzXn3TrFJAeI7vv96kLNBAOjTBqw3w7c3CxNF9WDh1JKRSQDodrSkEjA45fKx5H34+14rpsx5E290YO7o/CT5QlaldoEsYUhQISgKyySW4lL8mA1iYfoQP7/9G9ExrAnygUo+A+x2+c36aLMAaHfJllyaX4q/Lj08eC/kwPxFIgPfvydCzAkC3q1RRICD75tF/BYJNJtw/XIXXT3/BzPWrCfB7Fdv1CXS42WQBS55DvuGi7f8ISHm/uIi5oA9+G50Av1u+DT1MIR5xFCI8raGl1iQBvyXPId/t0XYhDh+q2ZG07EvwO7u26hKYspeUKAqMyI7aT23uuMCgOTclvO/zLToEqHeP2fLNyQK1OQ75OQ8e2o0fT7swIE2fAt5n0i4gcrRP8Rj6a3McSud8JXhvWboOgTLli2jQbGA1wzUKiDw1M24zpaX8GfmqDWd9e7O8A5UZ8fZWGf7sqTLgdmUWbu2RmolbuzNxsyIDN8q341p1LvrrizClUmCKp44TWnK9tji9hyl81mMlsby9VhL99STCB0vUTn+Z0JOwnaLH7KWz4/ZSLO9kEwVR3cbre8zmaX+QLCViL8qM8PSEtstmCU51D5rN+p9kSxl3mjaIPOUUOeqFuiWnw2ITbSZWO+M2U5rIUVyEoy9FOGoqwtOzIke/FXn6N5GjR0WeuhDh6F0gVvlZvpb/bT4CqnkOR0C309AAAAAASUVORK5CYII=";
    case 'DOC':
      // 根据文件类型返回相应的 Base64 图片
      return "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAAACXBIWXMAAAsTAAALEwEAmpwYAAACyUlEQVR4nO2WX0hTcRTH55xOm3uUoEcVrZVkvdVLT/VUDz3ch3qoaDnJv1O3qURoBRoURJFUhBGEJm4258w/c04zzKl5c2PzHzUbm3PO7pzT9keX39gthECYhhcz9oEDF34czufyu+fcw2JFifIXJOZpDiSK2s/xslVVPKFSzb8qd7AYobKXEyceOhxX9OESt/DdXW6+Ts3N1X5LvN6FfTlvwRO1IumaEnyhAjtXtMxUECMz1LEloySnZDgYV6xHvHgA3MJ+cPN7kZCnxWYCWRqKyOqgiMwOF5HZ5iIEKhcheDNHZIRDbifSGu1E6mv7yYj1Y8rNYMuMiJWQ4JSOYOsCbhztpJDZvoAjahcEqnkcUjqR0exAunwWaY12pNTbgrsqkNpgw/8rEIltCey/NQ65YRFipXVD4En/LJpG55FU0EMLlCvG0TBo2xCQGlYgGVtGKelFycclFI94IB72oEjvRuGge3sCsWVGePwhDFlXaIHkcj1+rP96k2N3BmkBo20J+i9uZgTYMiP6Pi/DGwghvkSPM7UmBEPrWF8Hrrw0gZergW81hKc6CzNXwJYZcb9vnk5MuU3ihvorpud9cHmDeKC1QnCznz7LfkEy9xFeeDVDFzn7bBIqI4Um0oWeSTd6JimcryXps+OVug2BbmcQmnDMBdDlCKBrNhx+dNr96LT5ti+QXmOmi0hbrHB4gihvseCexgpqZRUVzVNYDoTAz1EzJ8CRkqC+r6HNvEiLnH5kwMU6E/2sMS/g/TTF/BzonlqiuyHcAcmyARys0tMCXv8aHnZbmJsD7N8CNVoHnTzh9NGDKKGgl76CMJeff/pDYMfbMFZC4tTjCSjGKEhbZjYmYXW7Bc2jTmRU6JgX4OzWKGbvxZ9R1l4QSNnKQsJiciWrt59g/ftLaUQQw83tTEsUtRE8UWt1klDZzhcq5iLnRYnC2pSfIuJVdhBWe04AAAAASUVORK5CYII=";
    default:
      return "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAAACXBIWXMAAAsTAAALEwEAmpwYAAABKklEQVR4nGNgGAWDFfg9/R/o/+z/cv+n/1cRg11PfJytUn+Lj2oO8H/6/3PAs///icXOB9//06q7fEm34pIgVRwQQILlIOxy6MN/7erL/zWrL52hiiMCSHbAx/861Zep54gAMhygW3WFeo4IINEBroc+/teruko9RwSQ6AD345//61ddxXREzaXjZOWOABIdEPDk/3/79a/+W817DMaW8x79t5j/6L/5nAdfzZc+3k17BzzDj0cdwEBpFFgdvvfffN9tojBILdWjgKN42n/WgslEYZDa4ZcGFGbv+i83YztBbLj18jB1QMBARwHHQCdCq4HOhlZEOsDpwquBjQKF2buGaSIMGHkOePr/E9Uc8PT/J3Ic4E9KxwQnfvZ/ud+T/34kO2AUMNAJAABtYcoqssYmawAAAABJRU5ErkJggg==";  
  }
}
const handleKeyDown = (event: { key: string; }) => {
  if(list.value != null) {
    if(list.value?.length === 0) return; 
    if (event.key === 'ArrowUp') {
      selectIndex.value = selectIndex.value -1;
      if(selectIndex.value < 0) {
        selectIndex.value = list.value.length - 1;
      }
    } else if (event.key === 'ArrowDown') {
      selectIndex.value = selectIndex.value + 1;
      if(selectIndex.value >= list.value.length) {
        selectIndex.value = 0;
      }
    }

    // if(event.key === 'Enter') {
    //   handleOpen(list.value[selectIndex.value]);
    // }
  }
}

const handleMouseOver = (index: number) => {
  // Your logic for mouseover event
  selectIndex.value = index;
}

const handleOpen = (app: Application) => {
  invoke("open", { rType: app.r_type, path: app.r_exe_path, directive: directive.value });
}

const highlightText = (text: string) => {
  const regex = new RegExp(searchval.value || '', 'gi');
  return text.replace(regex, '<span class="highlight">$&</span>');
}
</script>

<template>
  <div class="result">
    <div class="loading"></div>
    <div class="list">
      <div class="list-item" :class="{ active: selectIndex === index }" :style="{ background: selectIndex === index? '#f7f7f7' : '' }" :index="index" v-for="(item, index) in list" :key="index" @click="handleOpen(item)" @mouseover="handleMouseOver(index)">
        <div class="icon">
          <img :src="fileToUrl(item.r_icon_path, item.r_type)" />
        </div>
        <div class="content">
          <div class="title">
            <span v-html="highlightText(item.text_name)"></span>
          </div>
          <div class="description">
            {{ item.r_exe_path }}
          </div>
        </div>
      </div>
    </div>
  </div>

</template>
<style>
.result {
  width: 100%;
  max-height: 315px;
  overflow-y: auto;
}
.loading {
  background: #fff;
  position: fixed;
  top: 0;
  left: 0;
  z-index: 999
}
.list {
  background-color: white;
}

.list-item {
  display: flex;
  align-items: center;
  padding: 15px;
  border-bottom: 1px solid rgb(243, 241, 241);
  overflow: hidden;
}

.icon {
  width: 32px;
  height: 32px;
  margin-right: 10px;
}

.content {
  display: flex;
  flex-direction: column;
}

.title {
  font-size: 14px !important;
}

.description {
  font-size: 12px;
  margin-top: 5px;
  color: gray;
}

.highlight {
  color: rgb(231, 134, 36);
}
</style>
