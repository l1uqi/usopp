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
    case 'JPG':
      // 根据文件类型返回相应的 Base64 图片
      return "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAAACXBIWXMAAAsTAAALEwEAmpwYAAACB0lEQVR4nO3T/0sTcRzH8f2eWWbf/L7m1rBSlH7wK0aE0g+B+6V/o3+ifsjQQouQqU1tMHV+qUiCtIK+YEQFg36obc5t7u62u+XWbXO6nC/xstiFcrfts/vFveH523HvB2/uVKr85Oe/MdAY6qKwaaABku2+c1AlNV05WJ6KkAQYcrT8bzkDXGe2MBoKY/4Xh1tcTHnAG54F1l3/urkPImeAWNwtAuxcQlGAI0qJAKbVsLKAG4EN2KOUcIlXPCt8E0QBnd8TqJ8PoWEhhKuO38r+BVe+rUM/w0I3FRDSP2HR+SOhDOCyLQ6d1Q/thLiz0wHhKqnPXvMkhYgB2j/HUD3OQGPZO63Vj45dxKWva6h78RO1c0G0f1nLHtD2KQq1mZZMM86g8R2P+perfwDPgzj/lEPrx2jmgObFCCrHKNlVPaZwYS4oAtTMsmhZjKQPuPg6jHKTL+0qRnw494wTAfRTATR/iKQHKBteQaZVPPKhZoYVAXQTfjS95+UDSoxeZFPp0IqwOBWgtTBofMvLA5wa8CDbThu90E36RQCNmRYuIQk48dANEp0c8KDawogA6lFKGlB8fxmkOv7AjTNmOj1AUZ8LJDvWvwz1GCUfcPTuEkhXdM+FKpNPHqCwx5Es7HGCdEd6l1A+7E1KAg51240Ft+2bBd0OkOzwHSeK+1w2SUB+DtxsA0B9iY9TrVvKAAAAAElFTkSuQmCC";
    case 'PNG':
      // 根据文件类型返回相应的 Base64 图片
      return "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAAACXBIWXMAAAsTAAALEwEAmpwYAAACB0lEQVR4nO3T/0sTcRzH8f2eWWbf/L7m1rBSlH7wK0aE0g+B+6V/o3+ifsjQQouQqU1tMHV+qUiCtIK+YEQFg36obc5t7u62u+XWbXO6nC/xstiFcrfts/vFveH523HvB2/uVKr85Oe/MdAY6qKwaaABku2+c1AlNV05WJ6KkAQYcrT8bzkDXGe2MBoKY/4Xh1tcTHnAG54F1l3/urkPImeAWNwtAuxcQlGAI0qJAKbVsLKAG4EN2KOUcIlXPCt8E0QBnd8TqJ8PoWEhhKuO38r+BVe+rUM/w0I3FRDSP2HR+SOhDOCyLQ6d1Q/thLiz0wHhKqnPXvMkhYgB2j/HUD3OQGPZO63Vj45dxKWva6h78RO1c0G0f1nLHtD2KQq1mZZMM86g8R2P+perfwDPgzj/lEPrx2jmgObFCCrHKNlVPaZwYS4oAtTMsmhZjKQPuPg6jHKTL+0qRnw494wTAfRTATR/iKQHKBteQaZVPPKhZoYVAXQTfjS95+UDSoxeZFPp0IqwOBWgtTBofMvLA5wa8CDbThu90E36RQCNmRYuIQk48dANEp0c8KDawogA6lFKGlB8fxmkOv7AjTNmOj1AUZ8LJDvWvwz1GCUfcPTuEkhXdM+FKpNPHqCwx5Es7HGCdEd6l1A+7E1KAg51240Ft+2bBd0OkOzwHSeK+1w2SUB+DtxsA0B9iY9TrVvKAAAAAElFTkSuQmCC";
    case 'GIF':
      // 根据文件类型返回相应的 Base64 图片
      return "data:image/png;base64,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";
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

    if(event.key === 'Enter') {
      handleOpen(list.value[selectIndex.value]);
    }
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
  width: 40px;
  margin-right: 10px;
}

.content {
  display: flex;
  flex-direction: column;
}

.title {
  font-size: 14px;
}

.description {
  font-size: 14px;
  margin-top: 5px;
  color: gray;
}

.highlight {
  color: rgb(231, 134, 36);
}
</style>
